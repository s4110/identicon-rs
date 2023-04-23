#[derive(Debug)]
pub struct HSL {
    hue: f32,
    sat: f32,
    lum: f32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Link: https://shanabrian.com/web/javascript/color-code-convert-hsl-to-10rgb.php
impl HSL {
    pub fn new(hue: f32, sat: f32, lum: f32) -> HSL {
        HSL { hue, sat, lum }
    }

    pub fn to_rgb(&self) -> RGB {
        let hue = self.hue / 360.0;
        let sat = self.sat / 100.0;
        let lum = self.lum / 100.0;

        let b = if lum <= 0.5 {
            lum * (sat + 1.0)
        } else {
            lum + sat - lum * sat
        };

        let a = lum * 2.0 - b;

        let red = HSL::hue_to_rgb(a, b, hue + 1.0 / 3.0);
        let green = HSL::hue_to_rgb(a, b, hue);
        let blue = HSL::hue_to_rgb(a, b, hue - 1.0 / 3.0);

        RGB {
            red: (red * 255.0).round() as u8,
            green: (green * 255.0).round() as u8,
            blue: (blue * 255.0).round() as u8,
        }
    }

    fn hue_to_rgb(a: f32, b: f32, hue: f32) -> f32 {
        let h = if hue < 0.0 {
            hue + 1.0
        } else if hue > 1.0 {
            hue - 1.0
        } else {
            hue
        };

        let result = if h < 1.0 / 6.0 {
            a + (b - a) * 6.0 * h
        } else if h < 1.0 / 2.0 {
            b
        } else if h < 2.0 / 3.0 {
            a + (b - a) * (2.0 / 3.0 - h) * 6.0
        } else {
            a
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{HSL, RGB};

    #[test]
    fn black() {
        let target = RGB {
            red: 0,
            green: 0,
            blue: 0,
        };
        let test_result = HSL::new(0.0, 0.0, 0.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn white() {
        let target = RGB {
            red: 255,
            green: 255,
            blue: 255,
        };
        let test_result = HSL::new(0.0, 0.0, 100.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn red() {
        let target = RGB {
            red: 255,
            green: 0,
            blue: 0,
        };
        let test_result = HSL::new(0.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn lime() {
        let target = RGB {
            red: 0,
            green: 255,
            blue: 0,
        };
        let test_result = HSL::new(120.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn blue() {
        let target = RGB {
            red: 0,
            green: 0,
            blue: 255,
        };
        let test_result = HSL::new(240.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn yellow() {
        let target = RGB {
            red: 255,
            green: 255,
            blue: 0,
        };
        let test_result = HSL::new(60.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn cyan() {
        let target = RGB {
            red: 0,
            green: 255,
            blue: 255,
        };
        let test_result = HSL::new(180.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn magenta() {
        let target = RGB {
            red: 255,
            green: 0,
            blue: 255,
        };
        let test_result = HSL::new(300.0, 100.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn silver() {
        let target = RGB {
            red: 191,
            green: 191,
            blue: 191,
        };
        let test_result = HSL::new(0.0, 0.0, 75.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn gray() {
        let target = RGB {
            red: 128,
            green: 128,
            blue: 128,
        };
        let test_result = HSL::new(0.0, 0.0, 50.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn maroon() {
        let target = RGB {
            red: 128,
            green: 0,
            blue: 0,
        };
        let test_result = HSL::new(0.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn olive() {
        let target = RGB {
            red: 128,
            green: 128,
            blue: 0,
        };
        let test_result = HSL::new(60.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn green() {
        let target = RGB {
            red: 0,
            green: 128,
            blue: 0,
        };
        let test_result = HSL::new(120.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn purpule() {
        let target = RGB {
            red: 127,
            green: 0,
            blue: 128,
        };
        let test_result = HSL::new(300.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn teal() {
        let target = RGB {
            red: 0,
            green: 128,
            blue: 127,
        };
        let test_result = HSL::new(180.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }

    #[test]
    fn navy() {
        let target = RGB {
            red: 0,
            green: 0,
            blue: 128,
        };
        let test_result = HSL::new(240.0, 100.0, 25.0).to_rgb();
        assert_eq!(target, test_result);
    }
}
