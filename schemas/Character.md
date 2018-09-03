# Character

|Key|Value|Description|
|---|---|---|
|`id`|`u64`|The character's ID on the Lodestone.|
|`name`|`String`|The character's name.|
|`world`|`String`|The world the character is on.|
|`race`|`Race`|The character's race. See Race section below.|
|`clan`|`Clan`|The character's clan. See Clan section below.|
|`gender`|`Gender`|The character's gender. See Gender section below.|
|`title`|`String?`|The character's title.|
|`name_day`|`String`|The character's birth date, in the form `"18th Sun of the 2nd Umbral Moon"`.|
|`guardian`|`Guardian`|The character's guardian deity. See Guardian section below.|
|`city_state`|`CityState`|The character's starting city-state. See CityState section below.|
|`grand_company`|`GrandCompanyInfo?`|The character's Grand Company affiliation and rank. See GrandCompanyInfo section below.|
|`free_company_id`|`u64?`|The ID of the character's Free Company, if any.|
|`profile_text`|`String`|The profile text for this character on the Lodestone. If empty, this will be the empty string (`""`).|

## Race

An enumerated type represented as a string.

|Race|String|
|---|---|
|Au Ra|`"AuRa"`|
|Elezen|`"Elezen"`|
|Hyur|`"Hyur"`|
|Lalafell|`"Lalafell"`|
|Miqo'te|`"Miqote"`|
|Roegadyn|`"Roegadyn"`|

## Clan

An enumerated type represented as a string.

|Clan|String|
|---|---|
|Raen|`"Raen"`|
|Xaela|`"Xaela"`|
|Duskwight|`"Duskwight"`|
|Wildwood|`"Wildwood"`|
|Highlander|`"Highlander"`|
|Midlander|`"Midlander"`|
|Dunesfolk|`"Dunesfolk"`|
|Plainsfolk|`"Plainsfolk"`|
|Seeker of the Moon|`"SeekerOfTheMoon"`|
|Seeker of the Sun|`"SeekerOfTheSun"`|
|Hellsguard|`"Hellsguard"`|
|Sea Wolf|`"SeaWolf"`|

## Gender

An enumerated type represented as a string.

|Gender|String|
|---|---|
|Male|`"Male"`|
|Female|`"Female"`|

## Guardian

An enumerated type represented as a string.

|Guardian|String|
|---|---|
|Althyk|`"Althyk"`|
|Azeyma|`"Azeyma"`|
|Byregot|`"Byregot"`|
|Halone|`"Halone"`|
|Llymlaen|`"Llymlaen"`|
|Menphina|`"Menphina"`|
|Nald'thal|`"NaldThal"`|
|Nophica|`"Nophica"`|
|Nymeia|`"Nymeia"`|
|Oschon|`"Oschon"`|
|Rhalgr|`"Rhalgr"`|
|Thaliak|`"Thaliak"`|

## CityState

An enumerated type represented as a string.

|City-state|String|
|---|---|
|Gridania|`"Gridania"`|
|Limsa Lominsa|`"LimsaLominsa"`|
|Ul'dah|`"UlDah"`|

## GrandCompanyInfo

|Key|Value|Description|
|---|---|---|
|`grand_company`|`GrandCompany`|The Grand Company. See GrandCompany section below.|
|`rank`|`String`|The character's rank in the Grand Company. Ex. `"Second Serpent Lieutenant"`|

## GrandCompany

An enumerated type represented as a string.

|Grand Company|String|
|---|---|
|Immortal Flames|`"Flames"`|
|Maelstrom|`"Maelstrom"`|
|Order of the Twin Adder|`"TwinAdders"`|
