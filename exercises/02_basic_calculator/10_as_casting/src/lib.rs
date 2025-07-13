// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() { 
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // `i8` value that is equivalent to `255` when converted to `u8`.
        let y: i8 = -1;
        // personal note: the i8 type goes from -128 to 127, including 0, so in this
        // case and considering that the exceding are 'wrapping', we have 255 - 127
        // = 128 starting from -128. At a first glance, I thought that the corresponding
        // value for `y` has to be 0 (128 - 128) but no! for instance, if x = 128 then
        // 128 - 127 = 1 and `y` has to be -128 (i.e., (1 - 1) - 128), so, finally, in
        // this case the answer is (128 - 1) - 128 = 127 - 128 = -1

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1; // 1 is true and 0 is false
        assert_eq!(true as u8, v);
    }
}
