# Weaponizing ISPs
We can all agree on online censorship by governments being a terrible idea. Now imagine if such tremendous power was given to some random private company. That would be even worst, right? Well, welcome to Italy, 2024!

## AGCOM Piracy Shield 🛡️
On July 14, 2023, AGCOM (the Italian Agency that oversees on online communications) deliberated the creation of an online platform, [the Piracy Shield](https://www.wired.it/article/piracy-shield-agcom-piattaforma-streaming-pirata-calcio-segnalazioni/) (link in Italian 🇮🇹 only, sorry), aiming to contrast online piracy. The idea is dead simple: private companies holding owner rights on digital content (read, streaming of online football - soccer - matches, a very big thing in Italy) can open a ticket to the platform, specifying an IPv4/IPv6 address or a FQDN (fully qualified domain name) guilty of distributing pirated content, and a 'proof' of copyright violation. ISPs in Italy are required to build integration with platform, and act in due diligence by replying to the above mentioned tickets within 30 minutes, blocking access to the target. What could go wrong?

## Why the Piracy Shield is an awful idea💡
There are a few (many) reasons why the Piracy Shield is not only an idea that could never work in practice, but it is also one of those very poorly implemented.


First off, blocking resources by DNS or IP address is extremely hard to get right in practice: say I open a ticket against 'pirate.com'. Major ISPs in Italy will either remove the DNS record for 'pirate.com' from their DNS servers, or point the domain to a stop page. How can I bypass this? Piece of cake: I just change the DNS server I am reaching out to another one, like Google, Cloudflare or OpenDNS. Bypassing the IP block gets slightly trickier, but a commercial VPN will very easily do the trick. Hence, the Piracy Shield already failed its purpose. Nice.

Now, an even more serious point. With this technology, Italy is *de-facto* weaponizing its ISPs and making them a censorship machine at the service of private companies - namely, Lega Serie A, Sky, Mediaset and DAZN - making up the Italian football lobby, that, by the way, also developed the platform in the first place and [gifted it to AGCOM](https://www.wired.it/article/piracy-shield-governo-politica-agcom/) (again Italian only 🇮🇹, sorry).


First of all, what is refered to 'proof' (or actually, [forensic evidence](https://github.com/fuckpiracyshield/service), according to some leaked code), it is a simple collection of hash digests, allegedly coming from the site distributing pirated content, and provided by the ticket opener. Its apparent use is for identification of the target and, probably, auditing. There is no verification going on whatsover to determine if the content distributed is indeed violating any copyright.
Furthermore, the operational constraints of the platform are non-sensical:

(1) Companies opening a ticket have 75s (only!) to make any correction to the ticket. After 24h, a ticket can be opened to cancel a previous one.

(2) ISPs have a 30m SLA to block the target.

(3) Any revocation can be done only after 6 months (by the law itself).

These constraints have led ISPs to implement an automated pipeline to block targets flagged by tickets on the platform, with little to no control. As a result, private companies have a way to censor basically any online resource without restrictions, through the forced compliance of ISPs. Of course, there is a caveat: the Piracy Shield comes with a whitelist maintained by AGCOM, which should filter out all nation critical services, to avoid major DoS attacks on our own infra. Spoiler alert: it doesn't.

<div style="display: flex; justify-content: center;">
  <img src="/assets/images/piracy_shield_login.png" alt="alt text" style="max-width: 100%; height: auto;" />
</div>

###### The ticket blocking one of Google Drive IPs 

## A carousel of incidents 🎪
The concerns of all the most prominent experts on the Italian IT panorama (shared also by less tech savvy people, but surprisingly not by AGCOM or the Piracy Shield creators), manifested in a series of embarrassing incidents. A big one already happened earlier this year, in February, when the mighty shield blocked a [Cloudflare IP](https://torrentfreak.com/agcom-admits-piracy-shield-blunder-cloudflare-urges-users-to-complain-240321/), resulting in a plethora of innocent websites being blocked for Italian users. The most prominent one, however, actually happened only yesterday (October 24, 2024), when a ticket blocked [one IP belonging to the Google CDN hosted in Milan](https://www.ansa.it/english/news/general_news/2024/10/20/anti-piracy-shield-halts-google-drive_2e7b137e-361d-4026-a704-9895c798e65a.html), where 70% of Google traffic in Italy goes through. Italian users were of course impacted, as many could not access Google Drive. Funnily enough, the block impacted DAZN, one of the football TV streamers with access to the Piracy Shield, as well. Also, it seems that even the stop page belonging to Tiscali - one of the biggest italian ISP - was blocked. Kinda hilarious, don't you think? 

<div style="display: flex; justify-content: center;">
  <img src="/assets/images/piracy_shield_1.jpeg" alt="alt text" style="max-width: 100%; height: auto;" />
</div>

<vspace></vspace>
<vspace></vspace>
<vspace></vspace>

<div style="display: flex; justify-content: center;">
  <img src="/assets/images/piracy_shield_2.jpeg" alt="alt text"style="max-width: 100%; height: auto;" />
</div>

This incident effectively shows two things:

(1) How error prone the ticket process is, how dangerous having a completely insecure and automated pipeline for blocking is, and how faulty the whitelist mechanism is (especially if [this](https://github.com/fuckpiracyshield/variations/blob/44ae7a0100598a7a7561e9eb331012aa8f518a1f/variations.py#L129) is what they came up with):

<div style="display: flex; justify-content: center;">
  <img src="/assets/images/piracy_shield_whitelist_code.png" alt="alt text"style="max-width: 100%; height: auto;" />
</div>

###### Some 'whitelisting' in the Piracy Shield code

(2) It is only a matter of time before the Piracy Shield backfires at the Italian government and AGCOM, possibly bringing down some critical service (think about an identity provider for SPID, the italian digital identity system used to access basically all government services, including healthcare or wealfare)
