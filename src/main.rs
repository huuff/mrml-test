use mrml::prelude::render::RenderOptions;

fn main() {
    let template = r#"
        <mjml>
        <mj-body>
            <mj-section>
            <mj-column>
                <mj-social font-size="15px" icon-size="30px" mode="horizontal">
                <mj-social-element name="facebook" href="https://mjml.io/">
                    Facebook
                </mj-social-element>
                <mj-social-element name="google" href="https://mjml.io/">
                    Google
                </mj-social-element>
                <mj-social-element name="twitter" href="https://mjml.io/">
                    Twitter
                </mj-social-element>
                <mj-social-element name="x" href="https://mjml.io/">
                    X
                </mj-social-element>
                </mj-social>
            </mj-column>
            </mj-section>
        </mj-body>
        </mjml>
    "#;

    let output = mrml::parse(template).unwrap();
    let output = output.element.render(&RenderOptions::default()).unwrap();

    println!("{output}");
}
