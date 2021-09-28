use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Local(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Domain(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    local: Local,
    domain: Domain,
}

pub fn local(input: &str) -> IResult<&str, Local> {
    // Take until the "+"
    let (input, name) = take_until("+")(input)?;
    // Consume the filter
    let (input, _) = take_until("@")(input)?;

    Ok((
        input,
        Local(name.chars().filter(|&ch| ch != '.').collect::<String>()),
    ))
}

pub fn email(input: &str) -> IResult<&str, Email> {
    let (input, local) = local(input)?;
    let (domain, _) = char('@')(input)?;

    Ok((
        input,
        Email {
            local,
            domain: Domain(domain.to_string()),
        },
    ))
}
