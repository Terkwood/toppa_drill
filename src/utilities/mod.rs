pub mod spritesheet_loading;

pub const STD_MARGIN: f32 = 1.0 / 256.0;

pub fn nearly_equal(float1: f32, float2: f32, margin: f32) -> bool{
    let mut equality: bool = false;
    if float1 > (float2 - margin) && float1 < (float2 + margin){
        equality = true;
    }

    equality
}