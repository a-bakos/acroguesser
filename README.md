# ACROGUESSER game - practice project

Simple command line game. The player is presented with a randomly chosen journal
title and they need to guess the journal's acronym to earn points.

### MVP Plan

Init:

- Use an API endpoint to request a list of journals in json format
	- Data needed to start: journal acronym and title
- Store journals list locally ?

Gameplay:

- [done] Ask for player's name
- Select random title + corresponding acronym
	- Validate the acronym (trim, check length (4 char max), see if it starts with 0)
- [done] Show title to user
- Init std input loop to take guesses
- If guess matches, add points to player's profile

### Post-MVP ideas

- Build in helper function(s):
	- User can request a character (and earns less points)
	- Maybe show a journal cover (need to extend the API call for this)
	- Journal imprint
- Limit guesses and/or decrease points per iteration
- Store/update user game profile as text file
- Colorised terminal text
- Clear terminal on each game loop and show game details

- Game menu
	- new
		- max points / txt / json
	- load
		- txt / json
	- save
		- txt / json
	- user
		- name
		- points
		- history
	- reset history
