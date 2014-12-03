// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec::Vec;
use std::default::Default;
use std::collections::HashSet;
use uuid::Uuid;

use world::{Payloadable, World, RelativeCoord, TraversalDirection};
use world::TraversalDirection::*;
use zone::{Zone, Tile};

pub use self::FovType::*;

#[deriving(Clone, Encodable, Decodable, Copy, Show, PartialEq)]
pub enum FovType {
    Blocking,
    Transparent,
    Void
}
impl Default for FovType {
    fn default() -> FovType { FovType::Void }
}
impl FovType {
    pub fn allow_los(&self) -> bool {
        match *self {
            Transparent | Void => true,
            Blocking => false
        }
    }
}

pub trait FovItem {
    fn get_fov(&self) -> FovType;
}

pub fn compute<TWorldPayload, TZonePayload, TPayload: Send + Payloadable + FovItem>(
    world: &World<TWorldPayload, TZonePayload, TPayload>, focus: RelativeCoord, radius: uint,
            start_ang: &mut [f64], end_ang: &mut [f64])
                -> Vec<RelativeCoord> {
    let mut visible_tiles: HashSet<RelativeCoord> = HashSet::new();
    let mut pending_zones = vec!((focus.zone_id, (focus.lx, focus.ly),
                              (focus.gx, focus.gy), radius, Uuid::nil(), NoDirection));
    let octants = [
        ((1, 1), true),   // 0 - SE-vert
        ((1, 1), false),  // 1 - SE-horiz
        ((1, -1), true),  // 2 - NE-vert
        ((1, -1), false), // 3 - NE-horiz
        ((-1, 1), true),  // 4 - SW-vert
        ((-1, 1), false), // 5 - SW-horiz
        ((-1, -1), true), // 6 - NW-vert
        ((-1, -1), false) // 7 - NW-horiz
    ];
    while pending_zones.len() > 0 {
        let before_len = visible_tiles.len();
        let (curr_zid, curr_focus, curr_offset, max_radius,
             from_pid, from_dir) =
            pending_zones.pop().expect("fov::compute .. popping zone, shouldn't happen");
        // When processing a connected zone, we only do the half of the screen
        // that we'll see based on the direction into which we arrived at the portal
        let mut octants_slice = match from_dir {
            // originating zone.. process all quads
            NoDirection => octants.iter().filter(|_| { true }),
            // North - NW and NE quads
            North => octants.iter().filter(|o| {
                let &((_,y),_) = *o;
                y == -1
            }),
            // South - SW and SE quads
            South => octants.iter().filter(|o| {
                    let &((_,y),_) = *o;
                    y == 1
            }),
            // West - NW and SW quads
            West => octants.iter().filter(|o| {
                    let &((x,_),_) = *o;
                    x == -1
            }),
            // East - NE and SE quads
            East => octants.iter().filter(|o| {
                    let &((x,_),_) = *o;
                    x == 1
            }),
        };
        let zone = world.get_zone(&curr_zid);
        // always insert focus pos, then check for portal at starting pos
        if from_pid == Uuid::nil() {
            visible_tiles.insert(RelativeCoord::new(curr_zid, curr_focus, curr_offset));
            let curr_tile = zone.get_tile(curr_focus);
            match curr_tile.portal_id {
                Some(pid) => {
                        pending_zones.push(build_pending_zone_entry(
                            world, zone.id, pid, curr_offset, max_radius));
                },
                None => {}
            }
        }
        let mut in_fov: HashSet<int> = HashSet::new();
        for o in octants_slice {
            let (quadrant, is_vert) = *o;
            let (tiles, zones) = compute_octant(
                world, zone, curr_focus, curr_offset, max_radius, from_pid,
                &mut in_fov, start_ang, end_ang, quadrant, is_vert, from_dir);
            for t in tiles.into_iter() { visible_tiles.insert(t); }
            for z in zones.into_iter() { pending_zones.push(z); }
        }
        let found = visible_tiles.len() - before_len;
    }

    debug!("num of visible_tiles: {}", visible_tiles.len());
    visible_tiles.into_iter().collect()
}

fn min(a: int, b: int) -> int {
    if a < b { a } else { b }
}
fn max(a: int, b: int) -> int {
    if a > b { a } else { b }
}

type ComputeOctantPendingZones = (Uuid, (uint, uint), (int, int), uint, Uuid, TraversalDirection);

fn compute_octant<TWorldPayload, TZonePayload, TTilePayload: Send + Payloadable + FovItem>(
                world: &World<TWorldPayload, TZonePayload, TTilePayload>,
                zone: &Zone<TZonePayload, TTilePayload>,
                position: (uint, uint),
                offset: (int, int), max_radius: uint, from_pid: Uuid,
                in_fov: &mut HashSet<int>,
                start_angle: &mut [f64], end_angle: &mut [f64],
                dn: (int, int), is_vert: bool, from_dir: TraversalDirection)
        -> (Vec<RelativeCoord>,
            Vec<ComputeOctantPendingZones>) {
    let mut visible_tiles = HashSet::new();
    let mut pending_zones = HashSet::new();
    let stub_tile = Tile::stub();
    let padding = 34 as int;
    let (raw_px, raw_py) = position;
    let (raw_px, raw_py) = (raw_px as int, raw_py as int);
    let (in_ox, in_oy) = offset;
    let wsize = zone.size;
    let wsize_sq = wsize * wsize;
    let (position_x, position_y) = match from_dir {
        NoDirection => (raw_px, raw_py),
        _ => {
            let (px, py) = (raw_px - in_ox, raw_py - in_oy);
            (px, py)
        }
    };
    let (dx, dy) = dn;
    {
        let mut iteration = 1 as int;
        let mut done = false;
        let mut total_obstacles = 0;
        let mut obstacles_in_last_line = 0;
        let mut min_angle = 0.0;
        // do while there are unblocked slopes left and the algo is within
        // the map's boundaries
        // scan progressive lines/columns from the focal-point outwards

        // branch:0 initial x,y values + initial bounds check for outer
        let mut x = if is_vert {
            0 as int
        } else {
            (position_x as int + dx) as int
        };
        // branch:0
        let mut y = if is_vert {
            (position_y as int + dy) as int
        } else {
            0 as int
        };
        // branch:0
        if is_vert {
            if y < -padding || y >= (wsize as int)+padding {
                    debug!("vdt: starting y:{} < 0 || y >= wisze", y);
                    done = true; }
        } else {
            if x < -padding || x >= (wsize as int)+padding {
                debug!("hdt: starting x:{} < 0 || x >= wisze", x);
                done = true; }
        }
        while !done {
            // process cells in the line
            let slopes_per_cell = 1.0 / (iteration as f64 + 1.0);
            let half_slopes = slopes_per_cell * 0.5;
            let mut processed_cell = (min_angle / slopes_per_cell) as int;
            done = true;
            // branch:1 calculate min/max inner bounds + set inner
            let (mini, maxi) = if is_vert {
                let minx = max(-padding, position_x as int - iteration);
                let maxx = min((wsize as int+padding) - 1, position_x as int + iteration);
                (minx, maxx)
            } else {
                let miny = max(-padding, position_y as int - iteration);
                let maxy = min((wsize as int+padding) - 1, position_y as int + iteration);
                (miny, maxy)
            };
            // branch:1
            let mut inner = if is_vert {
                x = (position_x as int + (processed_cell * dx)) as int;
                x
            } else {
                y = (position_y as int + (processed_cell * dy)) as int;
                y
            };
            while inner >= mini && inner <= maxi {
                let c = x + (y * wsize as int);
                let in_bounds = x >= 0 && y >= 0 && zone.coords_in_bounds((x as uint, y as uint));
                let c_tile = if in_bounds {
                    zone.tile_at_idx(c as uint)
                } else {
                    &stub_tile
                };
                let is_void = match c_tile.payload.get_fov() {
                    Void => true,
                    _ => false };
                let mut allow_los = c_tile.payload.get_fov().allow_los();
                let mut visible = true;
                let start_slope = processed_cell as f64 * slopes_per_cell;
                let center_slope = start_slope + half_slopes;
                let end_slope = start_slope + slopes_per_cell;
                if obstacles_in_last_line > 0 && !in_fov.contains(&c) {
                    let mut idx = 0;
                    while in_bounds && visible && idx < obstacles_in_last_line {
                        if allow_los {
                            if center_slope > start_angle[idx] &&
                                center_slope < end_angle[idx] {
                                visible = false;
                            }
                        } else if (start_slope >= start_angle[idx]) &&
                            (end_slope <= end_angle[idx]) {
                            visible = false;
                        }
                        // branch:2 zy vals +  n - dn bounds checks
                        let zy = if is_vert {
                            x + ((y-dy) * wsize as int)
                        } else {
                            x-dx + (y* wsize as int)
                        };
                        // branch:2
                        let n_minus_dn_bounds_check = if is_vert {
                            (x - dx >= 0) && (x - dx < wsize as int)
                        } else {
                            (y - dy >= 0) && (y - dy < wsize as int)
                        };
                        let zy_tile_trans = if zy >= 0 && zy < wsize_sq as int {
                            let t = zone.tile_at_idx(zy as uint);
                            t.payload.get_fov().allow_los()
                        } else { true };
                        let zyx = (x-dx) + ((y-dy) * wsize as int);
                        let zyx_tile_trans = if zyx >= 0 && zyx < wsize_sq as int {
                            let t = zone.tile_at_idx(zyx as uint);
                            t.payload.get_fov().allow_los()
                        } else { true };
                        if visible &&
                            (!in_fov.contains(&zy) || !zy_tile_trans) &&
                            (n_minus_dn_bounds_check && ((!in_fov.contains(&zyx)) ||
                                (!zyx_tile_trans))) {
                            visible = false;
                        }
                        idx += 1;
                    }
                }
                if is_void {
                    visible = false;
                    done = false;
                }
                let mut non_blocking_axis = true;
                match from_dir {
                    North => {
                        if y == raw_py && x != raw_px {
                            non_blocking_axis = false;
                            visible = true;
                            done = true;
                            allow_los = false;
                        } else if y > raw_py {
                            visible = false;
                            done = false;
                        }
                    },
                    South => {
                        if y == raw_py && x != raw_px {
                            non_blocking_axis = false;
                            visible = true;
                            done = true;
                            allow_los = false;
                        } else if y < raw_py {
                            visible = false;
                            done = false;
                        }
                    },
                    East => {
                        if x == raw_px && y != raw_py {
                            non_blocking_axis = false;
                            visible = true;
                            done = true;
                            allow_los = false;
                        } else if x < raw_px {
                            visible = false;
                            done = false;
                        }
                    },
                    West => {
                        if x == raw_px && y != raw_py {
                            non_blocking_axis = false;
                            visible = true;
                            done = true;
                            allow_los = false;
                        } else if x > raw_px {
                            visible = false;
                            done = false;
                        }
                    },
                    _ => {}
                }
                if visible {
                    let (gx, gy) = offset;
                    let (ox, oy) = (x - raw_px, y - raw_py);
                    let this_gx = (ox+gx, oy+gy);
                    let found_already = in_fov.contains(&c);
                    if non_blocking_axis {
                        in_fov.insert(c);
                    }
                    let add_this_tile = match c_tile.portal_id {
                        Some(pid) => {
                            if pid != from_pid && !found_already {
                                let iter = if iteration < 0 { 0 }
                                else { iteration };
                                let remaining_radius = max_radius as int - iter;
                                let remaining_radius = if remaining_radius < 0 {
                                    0 as uint
                                } else { remaining_radius as uint };
                                let remaining_radius = if from_pid == Uuid::nil() {
                                    max_radius
                                } else { remaining_radius };
                                let pz = build_pending_zone_entry(
                                    world, zone.id, pid, this_gx, remaining_radius
                                );
                                pending_zones.insert(pz);
                                false
                            } else { true }
                        },
                        None => true
                    };
                    if non_blocking_axis && add_this_tile {
                        visible_tiles.insert(RelativeCoord::new(
                            zone.id, (x as uint, y as uint),
                            this_gx));
                    }
                    done = false;
                    if !allow_los {
                        // update angle state..
                        if min_angle >= start_slope { min_angle = end_slope; }
                        else {
                            start_angle[total_obstacles] = start_slope;
                            end_angle[total_obstacles] = end_slope;
                            total_obstacles += 1;
                        }
                    }
                }
                processed_cell += 1;
                // branch:3 update x||y and inner
                if is_vert {
                    x += dx;
                    inner = x;
                } else {
                    y += dy;
                    inner = y;
                }
            }
            if iteration == max_radius as int {
                debug!("vdt: iteration == max_radius");
                done = true;
            }
            iteration += 1;
            obstacles_in_last_line = total_obstacles;
            // branch:4 update x||y + done
            if is_vert {
                y += dy;
                if y < -padding || y >= (wsize as int)+padding {
                    done = true;
                }
            } else {
                x += dx;
                if x < -padding || x >= (wsize as int)+padding {
                    done = true;
                }
            }
            if min_angle == 1.0 {
                done = true; }
        }
    }
    (visible_tiles.into_iter().collect(),
     pending_zones.into_iter().collect())
}

fn build_pending_zone_entry<TWorldPayload, TZonePayload, TPayload: Send + Payloadable + FovItem>(
    world: &World<TWorldPayload, TZonePayload, TPayload>, zid: Uuid, pid: Uuid, this_gx: (int, int),
    remaining_radius: uint)
        -> (Uuid, (uint, uint), (int, int), uint, Uuid, TraversalDirection) {
    let portal = world.get_portal(pid);
    let (ozid, from_dir) = portal.info_from(zid);
    let other_zone = world.get_zone(&ozid);
    let oc = other_zone.get_portal_coords(&pid);
    (ozid, *oc, this_gx, remaining_radius, pid, from_dir)
}
