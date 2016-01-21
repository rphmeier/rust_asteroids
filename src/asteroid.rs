pub struct Asteroid {
    position: (f32, f32),
    velocity: (f32, f32),
    radius: f32,
}

impl Asteroid {
    pub fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    pub fn pos(&self) -> (f32, f32) {
        self.position.clone()
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn still_alive(&self, width: f32, height: f32) -> bool {
        !(self.position.0 < -width || self.position.0 > width || self.position.1 < -height ||
          self.position.1 > height)
    }

    pub fn new() -> Asteroid {
        Asteroid {
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            radius: 10.0,
        }
    }

    pub fn new_with_attr(pos: (f32, f32), vel: (f32, f32), rad: f32) -> Asteroid {
        Asteroid {
            position: pos,
            velocity: vel,
            radius: rad,
        }
    }
}

pub const INDICES: [u16; 1074] = [0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 7, 0, 7, 8,
                                  0, 8, 9, 0, 9, 10, 0, 10, 11, 0, 11, 12, 0, 12, 13, 0, 13, 14,
                                  0, 14, 15, 0, 15, 16, 0, 16, 17, 0, 17, 18, 0, 18, 19, 0, 19,
                                  20, 0, 20, 21, 0, 21, 22, 0, 22, 23, 0, 23, 24, 0, 24, 25, 0,
                                  25, 26, 0, 26, 27, 0, 27, 28, 0, 28, 29, 0, 29, 30, 0, 30, 31,
                                  0, 31, 32, 0, 32, 33, 0, 33, 34, 0, 34, 35, 0, 35, 36, 0, 36,
                                  37, 0, 37, 38, 0, 38, 39, 0, 39, 40, 0, 40, 41, 0, 41, 42, 0,
                                  42, 43, 0, 43, 44, 0, 44, 45, 0, 45, 46, 0, 46, 47, 0, 47, 48,
                                  0, 48, 49, 0, 49, 50, 0, 50, 51, 0, 51, 52, 0, 52, 53, 0, 53,
                                  54, 0, 54, 55, 0, 55, 56, 0, 56, 57, 0, 57, 58, 0, 58, 59, 0,
                                  59, 60, 0, 60, 61, 0, 61, 62, 0, 62, 63, 0, 63, 64, 0, 64, 65,
                                  0, 65, 66, 0, 66, 67, 0, 67, 68, 0, 68, 69, 0, 69, 70, 0, 70,
                                  71, 0, 71, 72, 0, 72, 73, 0, 73, 74, 0, 74, 75, 0, 75, 76, 0,
                                  76, 77, 0, 77, 78, 0, 78, 79, 0, 79, 80, 0, 80, 81, 0, 81, 82,
                                  0, 82, 83, 0, 83, 84, 0, 84, 85, 0, 85, 86, 0, 86, 87, 0, 87,
                                  88, 0, 88, 89, 0, 89, 90, 0, 90, 91, 0, 91, 92, 0, 92, 93, 0,
                                  93, 94, 0, 94, 95, 0, 95, 96, 0, 96, 97, 0, 97, 98, 0, 98, 99,
                                  0, 99, 100, 0, 100, 101, 0, 101, 102, 0, 102, 103, 0, 103, 104,
                                  0, 104, 105, 0, 105, 106, 0, 106, 107, 0, 107, 108, 0, 108, 109,
                                  0, 109, 110, 0, 110, 111, 0, 111, 112, 0, 112, 113, 0, 113, 114,
                                  0, 114, 115, 0, 115, 116, 0, 116, 117, 0, 117, 118, 0, 118, 119,
                                  0, 119, 120, 0, 120, 121, 0, 121, 122, 0, 122, 123, 0, 123, 124,
                                  0, 124, 125, 0, 125, 126, 0, 126, 127, 0, 127, 128, 0, 128, 129,
                                  0, 129, 130, 0, 130, 131, 0, 131, 132, 0, 132, 133, 0, 133, 134,
                                  0, 134, 135, 0, 135, 136, 0, 136, 137, 0, 137, 138, 0, 138, 139,
                                  0, 139, 140, 0, 140, 141, 0, 141, 142, 0, 142, 143, 0, 143, 144,
                                  0, 144, 145, 0, 145, 146, 0, 146, 147, 0, 147, 148, 0, 148, 149,
                                  0, 149, 150, 0, 150, 151, 0, 151, 152, 0, 152, 153, 0, 153, 154,
                                  0, 154, 155, 0, 155, 156, 0, 156, 157, 0, 157, 158, 0, 158, 159,
                                  0, 159, 160, 0, 160, 161, 0, 161, 162, 0, 162, 163, 0, 163, 164,
                                  0, 164, 165, 0, 165, 166, 0, 166, 167, 0, 167, 168, 0, 168, 169,
                                  0, 169, 170, 0, 170, 171, 0, 171, 172, 0, 172, 173, 0, 173, 174,
                                  0, 174, 175, 0, 175, 176, 0, 176, 177, 0, 177, 178, 0, 178, 179,
                                  0, 179, 180, 0, 180, 181, 0, 181, 182, 0, 182, 183, 0, 183, 184,
                                  0, 184, 185, 0, 185, 186, 0, 186, 187, 0, 187, 188, 0, 188, 189,
                                  0, 189, 190, 0, 190, 191, 0, 191, 192, 0, 192, 193, 0, 193, 194,
                                  0, 194, 195, 0, 195, 196, 0, 196, 197, 0, 197, 198, 0, 198, 199,
                                  0, 199, 200, 0, 200, 201, 0, 201, 202, 0, 202, 203, 0, 203, 204,
                                  0, 204, 205, 0, 205, 206, 0, 206, 207, 0, 207, 208, 0, 208, 209,
                                  0, 209, 210, 0, 210, 211, 0, 211, 212, 0, 212, 213, 0, 213, 214,
                                  0, 214, 215, 0, 215, 216, 0, 216, 217, 0, 217, 218, 0, 218, 219,
                                  0, 219, 220, 0, 220, 221, 0, 221, 222, 0, 222, 223, 0, 223, 224,
                                  0, 224, 225, 0, 225, 226, 0, 226, 227, 0, 227, 228, 0, 228, 229,
                                  0, 229, 230, 0, 230, 231, 0, 231, 232, 0, 232, 233, 0, 233, 234,
                                  0, 234, 235, 0, 235, 236, 0, 236, 237, 0, 237, 238, 0, 238, 239,
                                  0, 239, 240, 0, 240, 241, 0, 241, 242, 0, 242, 243, 0, 243, 244,
                                  0, 244, 245, 0, 245, 246, 0, 246, 247, 0, 247, 248, 0, 248, 249,
                                  0, 249, 250, 0, 250, 251, 0, 251, 252, 0, 252, 253, 0, 253, 254,
                                  0, 254, 255, 0, 255, 256, 0, 256, 257, 0, 257, 258, 0, 258, 259,
                                  0, 259, 260, 0, 260, 261, 0, 261, 262, 0, 262, 263, 0, 263, 264,
                                  0, 264, 265, 0, 265, 266, 0, 266, 267, 0, 267, 268, 0, 268, 269,
                                  0, 269, 270, 0, 270, 271, 0, 271, 272, 0, 272, 273, 0, 273, 274,
                                  0, 274, 275, 0, 275, 276, 0, 276, 277, 0, 277, 278, 0, 278, 279,
                                  0, 279, 280, 0, 280, 281, 0, 281, 282, 0, 282, 283, 0, 283, 284,
                                  0, 284, 285, 0, 285, 286, 0, 286, 287, 0, 287, 288, 0, 288, 289,
                                  0, 289, 290, 0, 290, 291, 0, 291, 292, 0, 292, 293, 0, 293, 294,
                                  0, 294, 295, 0, 295, 296, 0, 296, 297, 0, 297, 298, 0, 298, 299,
                                  0, 299, 300, 0, 300, 301, 0, 301, 302, 0, 302, 303, 0, 303, 304,
                                  0, 304, 305, 0, 305, 306, 0, 306, 307, 0, 307, 308, 0, 308, 309,
                                  0, 309, 310, 0, 310, 311, 0, 311, 312, 0, 312, 313, 0, 313, 314,
                                  0, 314, 315, 0, 315, 316, 0, 316, 317, 0, 317, 318, 0, 318, 319,
                                  0, 319, 320, 0, 320, 321, 0, 321, 322, 0, 322, 323, 0, 323, 324,
                                  0, 324, 325, 0, 325, 326, 0, 326, 327, 0, 327, 328, 0, 328, 329,
                                  0, 329, 330, 0, 330, 331, 0, 331, 332, 0, 332, 333, 0, 333, 334,
                                  0, 334, 335, 0, 335, 336, 0, 336, 337, 0, 337, 338, 0, 338, 339,
                                  0, 339, 340, 0, 340, 341, 0, 341, 342, 0, 342, 343, 0, 343, 344,
                                  0, 344, 345, 0, 345, 346, 0, 346, 347, 0, 347, 348, 0, 348, 349,
                                  0, 349, 350, 0, 350, 351, 0, 351, 352, 0, 352, 353, 0, 353, 354,
                                  0, 354, 355, 0, 355, 356, 0, 356, 357, 0, 357, 358, 0, 358, 359];
