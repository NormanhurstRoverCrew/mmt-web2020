use yew::html;
use yew::Html;
pub fn crews_option() -> Html {
    CREWS
        .into_iter()
        .map(|crew| {
            html! {
                <option value={crew.clone()}>{crew.clone()}</option>
            }
        })
        .collect()
}

pub const CREWS: &'static [&'static str] = &[
    "Other",
    "Inter-state / Nar Nar Goon RC",
    "1st Albion Park Rover Crew",
    "1st Austinmer Rover Crew",
    "1st Caringbah Rover Crew",
    "1st Engadine Rover Crew",
    "1st Figtree Rover Crew",
    "1st Goulburn Rover Crew",
    "1st Haberfield Rover Crew",
    "1st Kentlyn Rover Crew",
    "1st Korrahill Rover Crew",
    "1st Merewerther Rover Crew",
    "1st Narwee Rover Crew",
    "1st Warilla Rover Crew",
    "1st/2nd Merrylands Rover Crew",
    "2nd Bankstown (LD Bach Vietnam) Rover Crew",
    "2nd Baulkham Hills Rover Crew",
    "2nd Castle Hill Rover Crew",
    "2nd Gordon Rover Crew",
    "2nd Orange Rover Crew",
    "2nd Queanbeyan Rover Crew",
    "3rd Rose Bay (Judean) Rover Crew",
    "4th Wollongong Rover Crew",
    "Abbotsford Rover Crew",
    "Albury Rovers",
    "Bathurst / Macquarie Rover Crew",
    "Berowra Rovers",
    "Blacktown Rover Crew",
    "Blaxland (Kalangadoo) Rovers",
    "Brush Park Rovers",
    "Cape Byron Rover Crew",
    "Coogee Rover Crew",
    "Dulwich Hill (Dame Dixson's Own) Rover Crew",
    "Epping Rovers",
    "Flat Gorge Rover Crew",
    "Forest Rovers",
    "Gosford Rovers",
    "Gymea Rover Crew",
    "Haweskbury River Rovers",
    "Hunter Valley Rover Crew",
    "Hurstville Rover Crew",
    "Kananga Rover Crew",
    "Kings Langley Rovers",
    "Kissing Point Rovers",
    "Lindfield Rovers",
    "Matong Rovers",
    "Mona Vale Rovers",
    "Mt Colah/Mt Ku-ring-gai Rovers",
    "Nepean Rover Crew",
    "Normanhurst Rovers",
    "North Lake Macquarie Rovers",
    "Oatley Bay Rover Crew",
    "Padstow Heights Rover Crew",
    "Platabeen Rover Crew",
    "Razorback Rovers",
    "Shoalhaven Rovers",
    "Tablelands Rover Crew",
    "Turramurra Rovers",
    "Wagga Wagga Rovers",
    "Wearne Bay Rover Crew",
    "Westmead Rover Crew",
    "Wyoming Rovers",
    "Yanco Agricultural High School Rover Crew",
    "Yaralla Rovers",
];
