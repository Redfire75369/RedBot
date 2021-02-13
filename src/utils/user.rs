use serenity::{
    model::{
        channel::Message,
        guild::Member
    },
    prelude::Context,
};

pub(crate) async fn user_from_str(word: &String, msg: &Message, ctx: &Context) -> (u64, String) {
    let mut error: String = String::from("");
    let id = if word.starts_with("<@!") && word.ends_with(">") {
        let str = &String::from(&(word[3..(word.len() - 1)]));
        if str.parse::<u64>().is_ok() {
            str.parse::<u64>().unwrap()
        } else {
            error = "Invalid User ID: ".to_owned() + str.as_str();
            0
        }
    } else if word.starts_with("<@") && word.ends_with(">") {
        let str = &String::from(&(word[2..(word.len() - 1)]));
        if str.parse::<u64>().is_ok() {
            str.parse::<u64>().unwrap()
        } else {
            error = "Invalid User ID: ".to_owned() + str.as_str();
            0
        }
    } else if word.parse::<u64>().is_ok() {
        word.parse::<u64>().unwrap()
    } else {
        error = String::from("Guild could not be retrieved");
        0

        /*
            let guild = msg.guild(&ctx.cache).await;

            // let members = ctx.http.get_guild_members(msg.guild_id.unwrap().0, Option::from(10), None).await;
            if guild.is_some() {
                let guild = guild.unwrap();
                let members = guild.members_starting_with(word, false, true).await;
                // let members = members_starting_with(members.unwrap(), word, false).await;
                if members.get(0).is_some() {
                    let member = &members.get(0).unwrap().0;
                    member.user.id.0
                } else {
                    error = "Invalid Username: ".to_owned() + word.as_str();
                    0
                }
            } else {
                error = String::from("Guild could not be retrieved");
                0
            }
        */
    };

    return (id, error);
}

async fn members_starting_with(all_members: Vec<Member>, prefix: &str, case_sensitive: bool) -> Vec<(Member, String)> {
    fn starts_with_case_insensitive(to_look_at: &str, to_find: &str) -> bool {
        to_look_at.to_lowercase().starts_with(&to_find.to_lowercase())
    }

    fn starts_with(prefix: &str, case_sensitive: bool, name: &str) -> bool {
        case_sensitive && name.starts_with(prefix)
            || !case_sensitive && starts_with_case_insensitive(name, prefix)
    }

    let members = all_members
        .into_iter()
        .filter_map(|member| {
            let username = &member.user.name;

            if starts_with(prefix, case_sensitive, username) {
                let username = username.to_string();
                Some((member, username))
            } else {
                let nick = member.nick.as_ref()?;
                if starts_with(prefix, case_sensitive, nick) {
                    let nick = nick.to_string();
                    Some((member, nick))
                } else {
                    None
                }
            }
        })
        .collect();

    members
}