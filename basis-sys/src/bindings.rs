/* automatically generated by rust-bindgen 0.56.0 */

pub type size_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_transcoder_state {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_transcoder {
    _unused: [u8; 0],
}
pub const block_format_cETC1: block_format = 0;
pub const block_format_cETC2_RGBA: block_format = 1;
pub const block_format_cBC1: block_format = 2;
pub const block_format_cBC3: block_format = 3;
pub const block_format_cBC4: block_format = 4;
pub const block_format_cBC5: block_format = 5;
pub const block_format_cPVRTC1_4_RGB: block_format = 6;
pub const block_format_cPVRTC1_4_RGBA: block_format = 7;
pub const block_format_cBC7: block_format = 8;
pub const block_format_cBC7_M5_COLOR: block_format = 9;
pub const block_format_cBC7_M5_ALPHA: block_format = 10;
pub const block_format_cETC2_EAC_A8: block_format = 11;
pub const block_format_cASTC_4x4: block_format = 12;
pub const block_format_cATC_RGB: block_format = 13;
pub const block_format_cATC_RGBA_INTERPOLATED_ALPHA: block_format = 14;
pub const block_format_cFXT1_RGB: block_format = 15;
pub const block_format_cPVRTC2_4_RGB: block_format = 16;
pub const block_format_cPVRTC2_4_RGBA: block_format = 17;
pub const block_format_cETC2_EAC_R11: block_format = 18;
pub const block_format_cETC2_EAC_RG11: block_format = 19;
pub const block_format_cIndices: block_format = 20;
pub const block_format_cRGB32: block_format = 21;
pub const block_format_cRGBA32: block_format = 22;
pub const block_format_cA32: block_format = 23;
pub const block_format_cRGB565: block_format = 24;
pub const block_format_cBGR565: block_format = 25;
pub const block_format_cRGBA4444_COLOR: block_format = 26;
pub const block_format_cRGBA4444_ALPHA: block_format = 27;
pub const block_format_cRGBA4444_COLOR_OPAQUE: block_format = 28;
pub const block_format_cRGBA4444: block_format = 29;
pub const block_format_cTotalBlockFormats: block_format = 30;
pub type block_format = ::std::os::raw::c_int;
pub const basis_tex_format_cETC1S: basis_tex_format = 0;
pub const basis_tex_format_cUASTC4x4: basis_tex_format = 1;
pub type basis_tex_format = ::std::os::raw::c_int;
pub const basis_texture_type_cBASISTexType2D: basis_texture_type = 0;
pub const basis_texture_type_cBASISTexType2DArray: basis_texture_type = 1;
pub const basis_texture_type_cBASISTexTypeCubemapArray: basis_texture_type = 2;
pub const basis_texture_type_cBASISTexTypeVideoFrames: basis_texture_type = 3;
pub const basis_texture_type_cBASISTexTypeVolume: basis_texture_type = 4;
pub const basis_texture_type_cBASISTexTypeTotal: basis_texture_type = 5;
pub type basis_texture_type = ::std::os::raw::c_int;
pub const transcoder_texture_format_cTFETC1_RGB: transcoder_texture_format = 0;
pub const transcoder_texture_format_cTFETC2_RGBA: transcoder_texture_format = 1;
pub const transcoder_texture_format_cTFBC1_RGB: transcoder_texture_format = 2;
pub const transcoder_texture_format_cTFBC3_RGBA: transcoder_texture_format = 3;
pub const transcoder_texture_format_cTFBC4_R: transcoder_texture_format = 4;
pub const transcoder_texture_format_cTFBC5_RG: transcoder_texture_format = 5;
pub const transcoder_texture_format_cTFBC7_RGBA: transcoder_texture_format = 6;
pub const transcoder_texture_format_cTFPVRTC1_4_RGB: transcoder_texture_format = 8;
pub const transcoder_texture_format_cTFPVRTC1_4_RGBA: transcoder_texture_format = 9;
pub const transcoder_texture_format_cTFASTC_4x4_RGBA: transcoder_texture_format = 10;
pub const transcoder_texture_format_cTFATC_RGB: transcoder_texture_format = 11;
pub const transcoder_texture_format_cTFATC_RGBA: transcoder_texture_format = 12;
pub const transcoder_texture_format_cTFFXT1_RGB: transcoder_texture_format = 17;
pub const transcoder_texture_format_cTFPVRTC2_4_RGB: transcoder_texture_format = 18;
pub const transcoder_texture_format_cTFPVRTC2_4_RGBA: transcoder_texture_format = 19;
pub const transcoder_texture_format_cTFETC2_EAC_R11: transcoder_texture_format = 20;
pub const transcoder_texture_format_cTFETC2_EAC_RG11: transcoder_texture_format = 21;
pub const transcoder_texture_format_cTFRGBA32: transcoder_texture_format = 13;
pub const transcoder_texture_format_cTFRGB565: transcoder_texture_format = 14;
pub const transcoder_texture_format_cTFBGR565: transcoder_texture_format = 15;
pub const transcoder_texture_format_cTFRGBA4444: transcoder_texture_format = 16;
pub const transcoder_texture_format_cTFTotalTextureFormats: transcoder_texture_format = 22;
pub const transcoder_texture_format_cTFETC1: transcoder_texture_format = 0;
pub const transcoder_texture_format_cTFETC2: transcoder_texture_format = 1;
pub const transcoder_texture_format_cTFBC1: transcoder_texture_format = 2;
pub const transcoder_texture_format_cTFBC3: transcoder_texture_format = 3;
pub const transcoder_texture_format_cTFBC4: transcoder_texture_format = 4;
pub const transcoder_texture_format_cTFBC5: transcoder_texture_format = 5;
pub const transcoder_texture_format_cTFBC7_M6_RGB: transcoder_texture_format = 6;
pub const transcoder_texture_format_cTFBC7_M5_RGBA: transcoder_texture_format = 6;
pub const transcoder_texture_format_cTFBC7_M6_OPAQUE_ONLY: transcoder_texture_format = 6;
pub const transcoder_texture_format_cTFBC7_M5: transcoder_texture_format = 6;
pub const transcoder_texture_format_cTFBC7_ALT: transcoder_texture_format = 7;
pub const transcoder_texture_format_cTFASTC_4x4: transcoder_texture_format = 10;
pub const transcoder_texture_format_cTFATC_RGBA_INTERPOLATED_ALPHA: transcoder_texture_format = 12;
pub type transcoder_texture_format = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_slice_info {
    pub m_orig_width: u32,
    pub m_orig_height: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_num_blocks_x: u32,
    pub m_num_blocks_y: u32,
    pub m_total_blocks: u32,
    pub m_compressed_size: u32,
    pub m_slice_index: u32,
    pub m_image_index: u32,
    pub m_level_index: u32,
    pub m_unpacked_slice_crc16: u32,
    pub m_alpha_flag: bool,
    pub m_iframe_flag: bool,
}
#[test]
fn bindgen_test_layout_basisu_slice_info() {
    assert_eq!(
        ::std::mem::size_of::<basisu_slice_info>(),
        52usize,
        concat!("Size of: ", stringify!(basisu_slice_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_slice_info>(),
        4usize,
        concat!("Alignment of ", stringify!(basisu_slice_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_orig_width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_orig_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_orig_height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_orig_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_num_blocks_x as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_num_blocks_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_num_blocks_y as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_num_blocks_y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_total_blocks as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_total_blocks)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_compressed_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_compressed_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_slice_index as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_slice_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_image_index as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_image_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_level_index as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_level_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_unpacked_slice_crc16 as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_unpacked_slice_crc16)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_alpha_flag as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_alpha_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_slice_info>())).m_iframe_flag as *const _ as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_slice_info),
            "::",
            stringify!(m_iframe_flag)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_image_info {
    pub m_image_index: u32,
    pub m_total_levels: u32,
    pub m_orig_width: u32,
    pub m_orig_height: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_num_blocks_x: u32,
    pub m_num_blocks_y: u32,
    pub m_total_blocks: u32,
    pub m_first_slice_index: u32,
    pub m_alpha_flag: bool,
    pub m_iframe_flag: bool,
}
#[test]
fn bindgen_test_layout_basisu_image_info() {
    assert_eq!(
        ::std::mem::size_of::<basisu_image_info>(),
        44usize,
        concat!("Size of: ", stringify!(basisu_image_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_image_info>(),
        4usize,
        concat!("Alignment of ", stringify!(basisu_image_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_image_index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_image_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_total_levels as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_total_levels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_orig_width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_orig_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_orig_height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_orig_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_width as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_height as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_num_blocks_x as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_num_blocks_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_num_blocks_y as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_num_blocks_y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_total_blocks as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_total_blocks)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_first_slice_index as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_first_slice_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_alpha_flag as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_alpha_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_info>())).m_iframe_flag as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_info),
            "::",
            stringify!(m_iframe_flag)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_image_level_info {
    pub m_image_index: u32,
    pub m_level_index: u32,
    pub m_orig_width: u32,
    pub m_orig_height: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_num_blocks_x: u32,
    pub m_num_blocks_y: u32,
    pub m_total_blocks: u32,
    pub m_first_slice_index: u32,
    pub m_alpha_flag: bool,
    pub m_iframe_flag: bool,
}
#[test]
fn bindgen_test_layout_basisu_image_level_info() {
    assert_eq!(
        ::std::mem::size_of::<basisu_image_level_info>(),
        44usize,
        concat!("Size of: ", stringify!(basisu_image_level_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_image_level_info>(),
        4usize,
        concat!("Alignment of ", stringify!(basisu_image_level_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_image_index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_image_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_level_index as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_level_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_orig_width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_orig_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_orig_height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_orig_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_width as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_height as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_num_blocks_x as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_num_blocks_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_num_blocks_y as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_num_blocks_y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_total_blocks as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_total_blocks)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_first_slice_index as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_first_slice_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_alpha_flag as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_alpha_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_image_level_info>())).m_iframe_flag as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_image_level_info),
            "::",
            stringify!(m_iframe_flag)
        )
    );
}
#[repr(C)]
pub struct basisu_file_info {
    pub m_version: u32,
    pub m_total_header_size: u32,
    pub m_total_selectors: u32,
    pub m_selector_codebook_size: u32,
    pub m_total_endpoints: u32,
    pub m_endpoint_codebook_size: u32,
    pub m_tables_size: u32,
    pub m_slices_size: u32,
    pub m_tex_type: basis_texture_type,
    pub m_us_per_frame: u32,
    pub m_slice_info: [u64; 3usize],
    pub m_total_images: u32,
    pub m_image_mipmap_levels: [u64; 3usize],
    pub m_userdata0: u32,
    pub m_userdata1: u32,
    pub m_tex_format: basis_tex_format,
    pub m_y_flipped: bool,
    pub m_etc1s: bool,
    pub m_has_alpha_slices: bool,
}
#[test]
fn bindgen_test_layout_basisu_file_info() {
    assert_eq!(
        ::std::mem::size_of::<basisu_file_info>(),
        112usize,
        concat!("Size of: ", stringify!(basisu_file_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_file_info>(),
        8usize,
        concat!("Alignment of ", stringify!(basisu_file_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_total_header_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_total_header_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_total_selectors as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_total_selectors)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_selector_codebook_size as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_selector_codebook_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_total_endpoints as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_total_endpoints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_endpoint_codebook_size as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_endpoint_codebook_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_tables_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_tables_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_slices_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_slices_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_tex_type as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_tex_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_us_per_frame as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_us_per_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_slice_info as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_slice_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_total_images as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_total_images)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_image_mipmap_levels as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_image_mipmap_levels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_userdata0 as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_userdata0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_userdata1 as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_userdata1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_tex_format as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_tex_format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_y_flipped as *const _ as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_y_flipped)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_etc1s as *const _ as usize },
        109usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_etc1s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisu_file_info>())).m_has_alpha_slices as *const _ as usize },
        110usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_file_info),
            "::",
            stringify!(m_has_alpha_slices)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisrs_vector_u32 {
    pub values: *const u32,
    pub size: size_t,
}
#[test]
fn bindgen_test_layout_basisrs_vector_u32() {
    assert_eq!(
        ::std::mem::size_of::<basisrs_vector_u32>(),
        16usize,
        concat!("Size of: ", stringify!(basisrs_vector_u32))
    );
    assert_eq!(
        ::std::mem::align_of::<basisrs_vector_u32>(),
        8usize,
        concat!("Alignment of ", stringify!(basisrs_vector_u32))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisrs_vector_u32>())).values as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisrs_vector_u32),
            "::",
            stringify!(values)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisrs_vector_u32>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisrs_vector_u32),
            "::",
            stringify!(size)
        )
    );
}
extern "C" {
    pub fn basisrs_file_info_get_mipmap_levels(data: *const basisu_file_info) -> basisrs_vector_u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisrs_vector_slice_info {
    pub values: *const basisu_slice_info,
    pub size: size_t,
}
#[test]
fn bindgen_test_layout_basisrs_vector_slice_info() {
    assert_eq!(
        ::std::mem::size_of::<basisrs_vector_slice_info>(),
        16usize,
        concat!("Size of: ", stringify!(basisrs_vector_slice_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basisrs_vector_slice_info>(),
        8usize,
        concat!("Alignment of ", stringify!(basisrs_vector_slice_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisrs_vector_slice_info>())).values as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisrs_vector_slice_info),
            "::",
            stringify!(values)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<basisrs_vector_slice_info>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basisrs_vector_slice_info),
            "::",
            stringify!(size)
        )
    );
}
extern "C" {
    pub fn basisrs_file_info_get_slice_info(data: *const basisu_file_info) -> basisrs_vector_slice_info;
}
extern "C" {
    pub fn basisrs_create_transcoder() -> *mut basisu_transcoder;
}
extern "C" {
    pub fn basisrs_destroy_transcoder(me: *mut basisu_transcoder);
}
extern "C" {
    pub fn basisrs_validate_file_checksums(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        full_validation: bool,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_validate_header(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_get_userdata(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        userdata0: *mut u32,
        userdata1: *mut u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_get_total_images(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> u32;
}
extern "C" {
    pub fn basisrs_get_tex_format(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> basis_tex_format;
}
extern "C" {
    pub fn basisrs_get_total_image_levels(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
    ) -> u32;
}
extern "C" {
    pub fn basisrs_get_image_level_desc(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
        level_index: u32,
        orig_width: *mut u32,
        orig_height: *mut u32,
        total_blocks: *mut u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_get_image_info(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_info: *mut basisu_image_info,
        image_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_get_image_level_info(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        level_info: *mut basisu_image_level_info,
        image_index: u32,
        level_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_get_file_info(
        me: *const basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        file_info: *mut basisu_file_info,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_start_transcoding(
        me: *mut basisu_transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_stop_transcoding(me: *mut basisu_transcoder) -> bool;
}
extern "C" {
    pub fn basisrs_get_ready_to_transcode(me: *const basisu_transcoder) -> bool;
}
extern "C" {
    pub fn basisrs_transcode_image_level(
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
        level_index: u32,
        pOutput_blocks: *mut ::std::os::raw::c_void,
        output_blocks_buf_size_in_blocks_or_pixels: u32,
        fmt: transcoder_texture_format,
        decode_flags: u32,
        output_row_pitch_in_blocks_or_pixels: u32,
        pState: *mut basisu_transcoder_state,
        output_rows_in_pixels: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisrs_find_slice(
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
        level_index: u32,
        alpha_data: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn basisrs_transcode_slice(
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        slice_index: u32,
        pOutput_blocks: *mut ::std::os::raw::c_void,
        output_blocks_buf_size_in_blocks_or_pixels: u32,
        fmt: block_format,
        output_block_stride_in_bytes: u32,
        decode_flags: u32,
        output_row_pitch_in_blocks_or_pixels: u32,
        pState: *mut basisu_transcoder_state,
        pAlpha_blocks: *mut ::std::os::raw::c_void,
        output_rows_in_pixels: u32,
        channel0: ::std::os::raw::c_int,
        channel1: ::std::os::raw::c_int,
    ) -> bool;
}
