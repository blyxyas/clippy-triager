//ISSUE #15042 - C-bug
fn main() {}

pub fn inner_type(ty: &Type) -> &Type {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(ty)) = args.args.first() {
                    return ty;
                }
            }
        }
    }
    panic!("Expected inner type");
}
