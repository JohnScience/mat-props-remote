use rust_xlsxwriter::*;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, Document, HtmlAnchorElement, Url, Window};

fn save_vec_u8_as_file(v: Vec<u8>, filename: &str) {
    let uint8array: js_sys::Uint8Array = unsafe { js_sys::Uint8Array::view(&v) };
    let uint8array: &JsValue = uint8array.as_ref();
    // creating blob from uint8array would lead to unpacking the array to string
    // i.e. if uint8array is [50, 73, ..] then blob would be "5073..".
    let buffer_source_seq = js_sys::Array::from_iter(std::iter::once(uint8array));
    let blob = Blob::new_with_buffer_source_sequence(&buffer_source_seq).unwrap();
    let url: String = Url::create_object_url_with_blob(&blob).unwrap();

    // FIXME: add URL and <a> cleanup

    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
    a.set_href(&url);
    a.set_download(filename);
    a.click();
}

fn download_results_for_names_and_vals<const N: usize>(names: [&'static str; N], vals: &[f64]) {
    let mut workbook = Workbook::new();
    let right_aligned_fmt = Format::new().set_align(FormatAlign::Right);

    let worksheet = workbook.add_worksheet();

    for (i, (name, val)) in names
        .iter()
        .copied()
        .zip(vals.iter().copied())
        .enumerate()
        .map(|(i, tup)| (i as u32, tup))
    {
        worksheet
            .write_string_with_format(i, 0, name, &right_aligned_fmt)
            .unwrap();
        worksheet.write_number(i, 1, val).unwrap();
    }

    // use save_to_writer to save into wrapper around js_sys::Array
    let buffer = workbook.save_to_buffer().unwrap();

    //let array = js_sys::Array::from_iter(buffer.into_iter().map(|x| JsValue::from(x)));
    save_vec_u8_as_file(buffer, "results.xlsx");
}

#[wasm_bindgen]
pub fn download_results_for_elastic_modules_for_unidirectional_composite(vals: &[f64]) {
    download_results_for_names_and_vals(
        ["E1", "E2", "E3", "v12", "v13", "v23", "G12", "G13", "G23"],
        vals,
    );
}

#[wasm_bindgen]
pub fn download_results_for_elastic_modules_for_honeycomb(vals: &[f64]) {
    download_results_for_names_and_vals(
        ["E1", "E2", "E3", "v12", "v13", "v23", "G12", "G13", "G23"],
        vals,
    );
}

#[wasm_bindgen]
pub fn download_results_for_thermal_conductivity_for_unidirectional_composite(vals: &[f64]) {
    download_results_for_names_and_vals(["K1", "K2", "K3"], vals);
}

#[wasm_bindgen]
pub fn download_results_for_thermal_expansion_for_unidirectional_composite(vals: &[f64]) {
    download_results_for_names_and_vals(["alpha1", "alpha2", "alpha3"], vals);
}

#[wasm_bindgen]
pub fn download_results_for_thermal_expansion_for_honeycomb(vals: &[f64]) {
    download_results_for_names_and_vals(["alpha1", "alpha2", "alpha3"], vals);
}
