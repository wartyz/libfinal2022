/*
    print()
    frameCount
    deltaTime
    focused
    cursor()
    frameRate()
    noCursor()
    displayWidth
    displayHeight
    windowWidth
    windowHeight
    windowResized()
    width
    height
    fullscreen()
    pixelDensity()
    displayDensity()
    getURL()
    getURLPath()
    getURLParams()
*/

use raylib::prelude::*;

// Función sólo de raylib
pub fn draw_fps_fi(d: &mut RaylibDrawHandle, x: f32, y: f32) {
    d.draw_fps(x as i32, y as i32);
}