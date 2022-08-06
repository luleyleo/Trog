fn main() {
    gio::compile_resources(
        "resources",
        "resources/resources.xml",
        "resources.gresource",
    );
}
