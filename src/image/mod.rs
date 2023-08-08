use exr::prelude::*;
use exr::meta::attribute::*;
use glam::Vec3;

/// Write an rgba exr file, generating the pixel values on the fly.
/// This streams the generated pixel directly to the file,
/// never allocating the actual total pixel memory of the image.
pub fn pts_to_exr( pts: Vec<Vec3>, file_path: String, resolution: usize ) {
    // this function can generate a color for any pixel
    let generate_pixels = |position: Vec2<usize>| {
        let i = position.x()+position.y()*resolution;
        //println!( "x: {:?} y: {:?}",position.x(),position.y() );
        let p = pts[i];
        (
            p.x, // red
            p.y, // green
            p.z, // blue
        )
    };

    let mut layer_attributes = LayerAttributes::named("generated rgba main layer");
    layer_attributes.comments = Some(Text::from("This image was generated as part of an example"));
    layer_attributes.software_name = Some(Text::from("EXRS Project"));
    layer_attributes.other.insert(
        Text::from("Layer Purpose (Custom Layer Attribute)"),
        AttributeValue::Text(Text::from("This layer contains the rgb pixel data"))
    );

    let layer = Layer::new(
        (resolution, resolution),
        layer_attributes,
        Encoding::SMALL_FAST_LOSSLESS, // use fast but lossy compression

        SpecificChannels::rgb(generate_pixels)
    );

    let mut image = Image::from_layer(layer);
    image.attributes.pixel_aspect = 1.0;

    image.attributes.other.insert(
        Text::from("Mice Count (Custom Image Attribute)"),
        AttributeValue::I32(23333)
    );

    // write it to a file with all cores in parallel
    image.write().to_file(file_path.clone().as_str()).unwrap();
    println!("created file {:?}", file_path);
}