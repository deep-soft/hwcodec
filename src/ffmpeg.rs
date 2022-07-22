use crate::*;
use serde_derive::{Deserialize, Serialize};
use std::os::raw::c_int;
use std::result::Result;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum DataFormat {
    H264,
    H265,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Vendor {
    NVIDIA,
    AMD,
    INTEL,
    OTHER,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AVHWDeviceType {
    AV_HWDEVICE_TYPE_NONE,
    AV_HWDEVICE_TYPE_VDPAU,
    AV_HWDEVICE_TYPE_CUDA,
    AV_HWDEVICE_TYPE_VAAPI,
    AV_HWDEVICE_TYPE_DXVA2,
    AV_HWDEVICE_TYPE_QSV,
    AV_HWDEVICE_TYPE_VIDEOTOOLBOX,
    AV_HWDEVICE_TYPE_D3D11VA,
    AV_HWDEVICE_TYPE_DRM,
    AV_HWDEVICE_TYPE_OPENCL,
    AV_HWDEVICE_TYPE_MEDIACODEC,
    AV_HWDEVICE_TYPE_VULKAN,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodecInfo {
    pub name: String,
    pub format: DataFormat,
    pub vendor: Vendor,
    pub score: i32,
    pub hwdevice: AVHWDeviceType,
}

impl Default for CodecInfo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            format: DataFormat::H264,
            vendor: Vendor::OTHER,
            score: Default::default(),
            hwdevice: AVHWDeviceType::AV_HWDEVICE_TYPE_NONE,
        }
    }
}

impl CodecInfo {
    pub fn score(coders: Vec<CodecInfo>) -> CodecInfos {
        let mut h264: Option<CodecInfo> = None;
        let mut h265: Option<CodecInfo> = None;

        for coder in coders {
            match coder.format {
                DataFormat::H264 => match &h264 {
                    Some(old) => {
                        if old.score < coder.score {
                            h264 = Some(coder)
                        }
                    }
                    None => h264 = Some(coder),
                },
                DataFormat::H265 => match &h265 {
                    Some(old) => {
                        if old.score < coder.score {
                            h265 = Some(coder)
                        }
                    }
                    None => h265 = Some(coder),
                },
            }
        }
        CodecInfos { h264, h265 }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodecInfos {
    pub h264: Option<CodecInfo>,
    pub h265: Option<CodecInfo>,
}

impl CodecInfos {
    pub fn serialize(&self) -> Result<String, ()> {
        match serde_json::to_string_pretty(self) {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }
    }

    pub fn deserialize(s: &str) -> Result<Self, ()> {
        match serde_json::from_str(s) {
            Ok(c) => Ok(c),
            Err(_) => Err(()),
        }
    }
}

pub fn ffmpeg_linesize_offset_length(
    pixfmt: AVPixelFormat,
    width: usize,
    height: usize,
    align: usize,
) -> Result<([usize; 2], [usize; 2], usize), ()> {
    let mut linesize: [c_int; 2] = [0; 2];
    let mut offset: [c_int; 2] = [0; 2];
    let mut length: [c_int; 1] = [0; 1];
    unsafe {
        if get_linesize_offset_length(
            pixfmt as _,
            width as _,
            height as _,
            align as _,
            linesize.as_mut_ptr(),
            offset.as_mut_ptr(),
            length.as_mut_ptr(),
        ) == 0
        {
            return Ok((
                linesize.map(|i| i as usize),
                offset.map(|i| i as usize),
                length[0] as usize,
            ));
        }
    }

    Err(())
}
