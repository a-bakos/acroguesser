# ACROGUESSER cli game - practice project - WIP

Simple command line game. The player is presented with a randomly chosen journal
title and they need to guess the journal's acronym to earn points.

Player has 5 tries per journal maximum and every try/guess decreases the earnable
points in that round.

### MVP Plan

**Init:**

- [done] Use an API endpoint to request a list of journals in json format
	- [done] Data needed to start: journal acronym and title
- [wip] Store journals list locally ?
- [done] Ask for player's name
- [done] Select random title + corresponding acronym
	- [done] Validate the acronym ([done]trim, [done]check length (4 char max), [done]see if it starts with 0)
- [done] Show title to user
- [done] Init std input loop to take guesses
- If guess matches, add points to player's profile
- [done] Clear console
- [wip] Use colored console prints
- create status_log.txt if it doesn't exist
- Build in helper function(s):
	- User can request a character (and earn less points in exchange)
	- Maybe show a journal cover (need to extend the API call for this)
	- Journal imprint?
- [done] Limit guesses and/or decrease points per iteration
- Store/update user game profile as text file
- Game stats in file:
	- number of games played
	- players

- Game menu
	- [done] new
		- max points / txt / json
	- load, continue
		- txt / json
	- save
		- txt / json
	- user
		- name
		- points
		- history
	- reset history
	- [done] exit
	- download data
