/*Instead of a single viewport that all cameras draw
to, introduce a Viewport component that specifies
a region of the screen a camera should draw to.
Each can have their own clear color, too.

If a camera doesn’t have a viewport then then
WebGlRender will just skip past it.*/

use specs::prelude::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}