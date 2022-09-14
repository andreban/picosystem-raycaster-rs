use micromath::F32Ext;

pub fn sins() -> &'static [f32; 360] {
    static mut SINS_INITIALIZED: bool = false;
    static mut SINS: [f32; 360] = [0.0_f32; 360];
    unsafe {
        if !SINS_INITIALIZED {
            
            for i in 0..360 {
                SINS[i] = f32::sin(i as f32);
            }
        }
        SINS_INITIALIZED = true;
        &SINS
    }
}

pub fn coss() -> &'static [f32; 360] {
    static mut COSS_INITIALIZED: bool = false;
    static mut COSS: [f32; 360] = [0.0_f32; 360];
    unsafe {
        if !COSS_INITIALIZED {
            
            for i in 0..360 {
                COSS[i] = f32::cos(i as f32);
            }
        }
        COSS_INITIALIZED = true;
        &COSS
    }
}

pub fn tans() -> &'static [f32; 360] {
    static mut TANS_INITIALIZED: bool = false;
    static mut TANS: [f32; 360] = [0.0_f32; 360];
    unsafe {
        if !TANS_INITIALIZED {
            
            for i in 0..360 {
                TANS[i] = f32::cos(i as f32);
            }
        }
        TANS_INITIALIZED = true;
        &TANS
    }
}
