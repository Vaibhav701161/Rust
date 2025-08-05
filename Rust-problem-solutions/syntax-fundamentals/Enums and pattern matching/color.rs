//21. Create a `Color` enum with RGB values and implement color mixing.

#[derive(Debug,Clone)]

enum Color{
    Red,
    Green,
    Blue,
    Custom(u8,u8,u8),
}

impl Color{
    fn mix(c1: &Color, c2: &Color) -> Color {
        use Color::*;

        match (c1,c2){
            (Red, Blue) => Color::Custom(128,0,128),
            (Red,Green) => Color::Custom(128,128,0),
            (Blue,Green) => Color::Custom(0,128,128),
            (Red, Red) => Red.clone(),
            (Greeen,Greeen)=> Green.clone(),
            (Blue, Blue) => Blue.clone(),
            (Custom(r1,g1,b1), Custom(r2,g2,b2) , base) => {
                Color::Custom((r1,g1,b1), Custom(r2,g2,b2)){
                    Color::Custom((r1+r2)/2, (g1+g2)/2, (b1+b2)/2)
                }

                (base, Custom(r,g,b)) | (Custom(r,g,b),base) => {
                    let (r1,g1,b1) = match base{
                        Red => (255,0,0),
                        Green => (0,255,0),
                        Blue => (0,0,255),
                        _ => (0,0,0),
                    };
                    Color::Custom((r1+r)/2, (g1+g)/2, (b1+b)/2)
                }
            }
        }
    }
}

