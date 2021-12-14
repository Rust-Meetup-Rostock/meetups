pub fn scoped() {
    let shadowed_binding = 1;

    {
        let shadowed_binding = "abc";
    }

    let shadowed_binding = 2;
}
