use yew::prelude::*;
use crate::components::svg::{
    emojis::{ Blushing, Mail },
    undraw::QuickChat,
};
use crate::components::socials::Socials;
use crate::ui::{Title, Card, TextLink};

#[function_component(Contact)]
pub fn contact() -> Html {
    fn handle_contact_description() -> Html {
		html!{
			<>
				<p>{ "My preferred way of contact is via " }
					<TextLink link="mailto:francesco.intoci@protonmail.com">{ "email" }<Mail /></TextLink>
				</p>
			</>
		}
    } 


    html!{
	<>
	    <Title id="contact">
                { "Contacts" } { " " } <Blushing />
        </Title>

	    <Card>
            <div class="flex flex-col gap-10">
                <div class="flex flex-wrap gap-8 items-center justify-center">
                    <QuickChat class="min-w-[200px] max-w-[400px] h-auto max-[300px]:min-w-full" />
                    <div class="flex-1 grow min-w-[300px] [&>p>svg]:h-[1.5rem] [&>p>a>svg]:h-[1.5rem] [&>p>a]:text-blue-700 [&>p>a]:underline [&>p>a]:dark:text-blue-400 flex flex-col gap-4 max-[850px]:min-w-full">
                        { handle_contact_description() }
                    </div>
                </div>

                <Socials />
            </div>
	    </Card>
        </>
    }
}
