// src/graphics/texture.rs

use std::ffi::CString;
use gl::types::*;
use image::{self, GenericImageView};

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new(image_path: &str) -> Self {
        let img = image::open(image_path).expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let img_data = img.to_rgba8().into_raw();

        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLint,
                height as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { id: texture_id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

