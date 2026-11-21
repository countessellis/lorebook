# Design Decisions

## On Elements

In generic terms, Lorebook is made up of elements. These are storytelling elements that make up the building blocks to tell the tales. Elements contain other elements

As other portions of the language and system are built, elements are basically the nouns, with the attributes of elements being the adjectives. The verb type parts of Lorebook will act on elements or allow elements to act.

Current elements:

* Library: A library is a set of tomes. It is container to allow multiple tomes. The only action with a library is to add or remove tomes from it and open a tome. Everything else happens at deeper levels.

* Tome: A tome is the book of stories. It contains a set of characters and settings that can be added to stories, and the stories themselves. Everything happens in the stories, but the characters and settings are what provide the building blocks.

* Stories: A story is the main unit of Lorebook. Stories are held in tomes, and everything important happens in the story.

* Settings: A setting is the location part of a story happens in. Characters are placed in settings, do their things, and might move from setting to setting within a story. A setting within a tome but not a story is potential, a place where something might happen. In a story, it becomes the location something actually can happen in.

* Characters: A character is the actor for a story. Not all characters are human or humanoid, or even animals or creatures. A character is anything in a story that can act, can conduct actions. Stories can't happen without characters, as characters are who drive the story forward. A character within a tome but not within a story is potential, an actor without a part to play, but available to play one. In a story, it becomes the mover of the story, a bringer of action, an actor in a role.

## On Uniqueness

Characters, settings, and stories are all created at the tome level.

Characters can be added to settings and stories, and can be exported out of and imported into tomes, settings, and stories.

Settings can be added to stories, and can be exported out of and imported into tomes and stories.

Characters and settings in elements that are imported are carried into those.

While it was an option to make the tome level authoritative and link with ids instead of including the elements in the more specific elements, this isn't how real stories work. Characters and settings change based on the story, so treating them as a shared element at the tome or library level limits the ability for the liminal mutations to occur. As such, while we will have these elements at each level, there on joining another element, there is a split from the original. While the character or setting may be akin to the same character or setting in a separate context, the context makes it unique.

Lorebook will not deconflict characters and setting, it will presume the author/programmer is keeping track of if they want characters and settings to be unique. IDs are used for the author/programmer to look up elements within an element, but are only valid within their own context.

As other elements beyond libraries, tomes, settings, and characters are added, they will follow the same principles as these elements.
