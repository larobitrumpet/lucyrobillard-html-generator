use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use chrono;
use chrono::Datelike;
use quick_xml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Link {
    href: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Entry {
    level: u32,
    indent_num: usize,
    title: String,
    description: Option<String>,
    link: Option<Link>,
    youtube: Option<String>,
    summary: Option<String>,
    entry: Option<Vec<Entry>>,
    miscellanious: Option<String>,
}

impl Entry {
    fn to_html(&self) -> String {
        let indent = "    ".repeat(self.indent_num);
        let level = self.level;
        let title = &self.title;
        format!("\
{indent}<h{level}>{title}</h{level}>{}{}{}{}{}",
            if let Some(des) = &self.description {
                format!("
{indent}<p>{des}</p>")
            } else {
                String::from("")
            },
            if let Some(link) = &self.link {
                format!("
{indent}<p><a target=\"_blank\" href=\"{}\">{}</a></p>",
                    link.href,
                    link.text
                )
            } else {
                String::from("")
            },
            if let Some(iden) = &self.youtube {
                format!("
{indent}<iframe width=\"560\" height=\"315\" src=\"https://www.youtube-nocookie.com/embed/{iden}\" frameborder=\"0\" allow=\"accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture\" allowfullscreen></iframe>")
            } else {
                String::from("")
            },
            if let Some(summary) = &self.summary {
                format!("
{indent}<details>
{indent}    <summary>{summary}</summary>
{}
{indent}</details>",
                    if let Some(entries) = &self.entry {
                        format!("\
{indent}    <ul>
{}
{indent}    </ul>",
                        entries.iter().map(|x| {
                            format!("\
{indent}        <li>
{}
{indent}        </li>",
                                x.to_html()
                            )}).collect::<Vec<String>>().join("\n")
                        )
                    } else {
                        String::from("")
                    }
                )
            } else {
                if let Some(entries) = &self.entry {
                    format!("
{indent}<div>
{indent}    <ul>
{}
{indent}    </ul>
{indent}</div>",
                        entries.iter().map(|x| {
                            format!("\
{indent}        <li>
{}
{indent}        </li>",
                                x.to_html()
                            )}).collect::<Vec<String>>().join("\n")
                    )
                } else {
                    String::from("")
                }
            },
            if let Some(mis) = &self.miscellanious {
                format!("{}", mis)
            } else {
                String::from("")
            }
        )
    }
}

#[derive(Debug, Deserialize)]
struct Document {
    title: String,
    description: String,
    body: Option<String>,
    entry: Option<Vec<Entry>>,
}

impl Document {
    fn to_html(&self) -> String {
        if let Some(body) = &self.body {
            format!("\
<!DOCTYPE html>
<html>
{}{}
</html>",
                Self::head(&self.title, &self.description), Self::body(
                    &format!("        {}{}",
                        body,
                        if let Some(entries) = &self.entry {
                            format!("\n{}", entries.iter().map(|x| x.to_html()).collect::<Vec<String>>().join("\n"))
                        } else {
                            String::from("")
                        }
                    )
                )
            )
        } else {
            String::from("")
        }
    }

    fn head(title: &str, description: &str) -> String {
        format!("\
<head>
    <title>{title}</title>
    <meta charset=\"UTF-8\">
    <meta name=\"description\" content=\"{description}\">
    <meta name=\"author\" content=\"Lucy Robillard\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link rel=\"stylesheet\" href=\"style.css\">
</head>
")
    }

    fn body(content: &str) -> String {
        format!("\
<body>
    <nav>
        <ul class=menu>
            <li><a href=\"index.html\">Home</a></li>
            <li><a href=\"projects.html\">Projects</a></li>
            <li><a href=\"music.html\">Music</a></li>
            <li><a href=\"notary.html\">Notary</a></li>
            <li><a href=\"resume.html\">Résumé</a></li>
            <li><a href=\"about.html\">About</a></li>
        </ul>
    </nav>
    <div class=\"content\">
{content}
    </div>
    <footer>
        <p>© 2020-{} Lucy Robillard</p>
        <p><a href=\"mailto:larobitrumpet@lucyrobillard.xyz\">larobitrumpet@lucyrobillard.xyz</a></p>
        <p><a rel=\"license\" href=\"http://creativecommons.org/licenses/by/4.0/\"><img alt=\"Creative Commons License\" style=\"border-width:0\" src=\"https://i.creativecommons.org/l/by/4.0/88x31.png\" /></a><br />This work is licensed under a <a rel=\"license\" target=\"_blank\" href=\"http://creativecommons.org/licenses/by/4.0/\">Creative Commons Attribution 4.0 International License</a>.</p>
        <p>The code for this website can be found here: <a target=\"_blank\" href=\"https://github.com/larobitrumpet/html\">github.com/larobitrumpet/html</a>.</p>
    </footer>
</body>",
            chrono::Local::now().year())
    }
}

fn load_xml(sourcefile: &str) -> Document {
    if let Ok(source) = fs::read_to_string(sourcefile) {
        quick_xml::de::from_str(&source).unwrap()
    } else {
        panic!("Cannot open file `{}`", sourcefile);
    }
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        panic!("Too few arguments");
    } else if args.len() == 2 {
        args.next();
        let doc = load_xml(&args.next().unwrap());
        println!("{}", doc.to_html());
    } else if args.len() == 3 {
        args.next();
        let doc = load_xml(&args.next().unwrap());
        let mut file = File::create(&args.next().unwrap()).unwrap();
        file.write_all(doc.to_html().as_bytes()).unwrap();
    } else {
        panic!("Too many arguments");
    }
}
