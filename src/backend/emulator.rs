use byteorder::{ByteOrder, LittleEndian};
use std::mem;
use std::ptr;
use std::slice::from_raw_parts;

static PATCH_FILES: [&'static [u8]; 26] = [
    include_bytes!("../patches/5b6000"),
    include_bytes!("../patches/5d4000"),
    include_bytes!("../patches/7b2000"),
    include_bytes!("../patches/7cb000"),
    include_bytes!("../patches/7ea000"),
    include_bytes!("../patches/29d35c"),
    include_bytes!("../patches/36a000"),
    include_bytes!("../patches/37a550"),
    include_bytes!("../patches/369e80"),
    include_bytes!("../patches/369f00"),
    include_bytes!("../patches/369f40"),
    include_bytes!("../patches/369f80"),
    include_bytes!("../patches/2773f8"),
    include_bytes!("../patches/2777c4"),
    include_bytes!("../patches/2794b0"),
    include_bytes!("../patches/3198B4"),
    include_bytes!("../patches/27761c"),
    include_bytes!("../patches/27770c"),
    include_bytes!("../patches/277184"),
    include_bytes!("../patches/277308"),
    include_bytes!("../patches/277520"),
    include_bytes!("../patches/279854"),
    include_bytes!("../patches/369400"),
    include_bytes!("../patches/369800"),
    include_bytes!("../patches/570000"),
    include_bytes!("../patches/591000"),
];

pub struct Emulator {
    base_addr: usize,
}

impl Emulator {
    pub fn new() -> Emulator {
        let base_addr = 0;
        // let base_addr = Emulator::get_base_addr().expect("Could not find base address");
        Emulator { base_addr }
    }

    pub fn patch(&self) {
        for patch in PATCH_FILES.iter() {}
    }

    fn get_base_addr() -> Result<usize, ()> {
        unsafe {
            for i in (0..0x72D0000).step_by(0x1000) {
                let mut data = mem::uninitialized();

                let address = &mut data as *mut _;

                ptr::write(address, i);
                println!("{0:p}: {0}", &data);

                // let p = 0x0 as *const u8;
                // let n = ptr::write(p);
                // println!("{}", n);

                let p = &address as *const _ as *const u32;
                let n = ptr::read(p);
                if n != 0 {
                    // println!("{}", n);
                }
                println!(
                    "ADR {:?} {:?} {:?}",
                    *(i as *const u8),
                    from_raw_parts(i as *const u8, 4),
                    LittleEndian::read_u32(from_raw_parts(&address as *const _ as *const u8, 4))
                );
                // let mem_val = LittleEndian::read_u32(from_raw_parts(i as *const u8, 4));
                // println!("mem val {}", mem_val);
                // if mem_val == 0x3C1A8032 {
                //     continue;
                // };
                // let mem_val = LittleEndian::read_u32(from_raw_parts((i + 4) as *const u8, 4));
                // if mem_val == 0x275A7650 {
                //     continue;
                // };
                // return Ok(i);
            }
        }
        Err(())
    }
}
