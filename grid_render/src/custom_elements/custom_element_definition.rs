pub trait CustomElementDefinition {
    /**
    Function that will run when the custom component appears in the html
    */
    fn define(document: &web_sys::Document, element: &web_sys::Element);
}
