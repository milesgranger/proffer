use proffer::*;
use syn::parse::Parse;
use syn::*;

pub trait Verify: SrcCode {
    type ExpectedType: Parse;

    fn verify_generated(src_code: &str) {
        syn::parse_str::<Self::ExpectedType>(src_code).unwrap();
    }

    fn generate_and_verify(&self) -> String {
        let src_code = self.generate();
        Self::verify_generated(&src_code);
        src_code
    }
}

impl Verify for Struct {
    type ExpectedType = ItemStruct;
}

impl Verify for Trait {
    type ExpectedType = ItemTrait;
}

impl Verify for Module {
    type ExpectedType = ItemMod;
}

impl Verify for Impl {
    type ExpectedType = ItemImpl;
}

impl Verify for Enum {
    type ExpectedType = ItemEnum;
}

impl Verify for Function {
    type ExpectedType = ItemFn;
}
