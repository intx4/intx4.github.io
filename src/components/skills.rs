use yew::prelude::*;

use crate::components::svg::emojis::Nerd;
use crate::ui::{Title, Card};

struct Skill {
    pub lang: String,
    pub img_path: String,
}

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills: Vec<Skill> = vec![
        Skill {
            lang: "JavaScript".into(),
            img_path: "./assets/images/tech-icons/js.png".into(),
        },
        // Add more skills as such ^
    ];

    html!{
	<>
            <Title>
                { "Skills" } { " " } <Nerd />
            </Title>

            <Card>
                <div class="flex flex-wrap gap-12 items-center justify-center">
                    {
                        skills.iter().map(|skill: &Skill| html! {
                           <div class="relative flex justify-center [&>span]:hover:block">
                                <img
                                    class="h-[5rem] rounded-lg opacity-85 max-[615px]:h-[3rem] max-[255px]:h-[2rem]"
                                    src={skill.img_path.clone()}
                                    alt={skill.lang.clone()}
                                    loading="lazy"
                                />
                                <span class="hidden animate-fadein absolute top-[4rem] max-[615px]:top-[2.5rem] max-[255px]:top-[1.5rem] py-1 px-2 bg-black/50 backdrop-blur-sm text-white rounded-lg select-none">
                                    { skill.lang.clone() }
                                </span>
                            </div>
                        }).collect::<Html>()
                    }
                </div>
            </Card>
        </>
    }
}
