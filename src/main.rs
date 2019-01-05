extern crate actix_web;
use actix_web::{server, App, Result, Path, HttpRequest, HttpResponse, Responder};

extern crate horaire;

use horaire::timelines::TimeLine;
use horaire::source::transilien::transilien;
use horaire::source::sncf::sncf;
use horaire::source::ratp::ratp;
// use horaire::errors::*;

fn get_time_lines_html<'a, I>(time_lines: I) -> String
where
    I: Iterator<Item = &'a TimeLine>,
{
    let mut strings = time_lines.fold(String::from("<html><head><meta charset=\"UTF-8\"></head><body>"), |acc, ref mut time_line| {
                        acc + &format!("{}<p>", time_line)
                    });
    strings.pop();
    strings.push_str("</body></html>");
    strings
    // time_lines.map(|time_line| format!("{}", time_line)).collect::<Vec<_>>().join("<p>\n")
}

//#[get("/transilien/<station>", format = "text/html")]
fn rt_transilien(info: Path<(String)>) -> Result<String> {
    Ok(get_time_lines_html(transilien(&info).unwrap().iter()))
}

//#[get("/ratp/<line>/<station>", format = "text/html")]
fn rt_ratp(info: Path<(String, String)>) -> String {
    get_time_lines_html(ratp(&info.0 /*line*/, &info.1 /*station*/).unwrap().iter())
}

//#[get("/sncf/dest/<station>", format = "text/html")]
fn rt_sncf_dest(info: Path<(String)>) -> String {
    get_time_lines_html(sncf(&info, true).unwrap().iter())
}

//#[get("/sncf/arriv/<station>", format = "text/html")]
fn rt_sncf_arriv(req: HttpRequest) -> impl Responder {
    match req.match_info().get("station") {
        Some(station) => match sncf(station, false) {
                    Ok(time_lines) => HttpResponse::Ok()
                       .content_type("text/html")
                       .body(get_time_lines_html(time_lines.iter())),
                    _ => HttpResponse::NotAcceptable().finish()
        },
        None => HttpResponse::BadRequest().finish()
    }
}

fn main() {

    server::new(
        || App::new()
            .resource("/transilien/{station}",  |r| r.with(rt_transilien))
            .resource("/ratp/{line}/{station}", |r| r.with(rt_ratp))
            .resource("/sncf/dest/{station}",   |r| r.with(rt_sncf_dest))
            .resource("/sncf/arriv/{station}",  |r| r.with(rt_sncf_arriv)))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}