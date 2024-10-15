# Did China break 'military-grade' encryption?
Do you all remember the fuss about China breaking RSA with a quantum computer, early in 2023? No? Well, that's not surprising, especially given that RSA is still around and has not been broken (yet). In case you did remember, you should also remember how quickly that news [was debunked by experts of the field](https://scottaaronson.blog/?p=6957).

Turns out we are back at it. A few days ago, I came across news of some researchers from reputable Chinese institutions (e.g., Shanghai University) allegedly breaking 'military-grade' encryption with the help of a D-Wave quantum annealing computer ([original paper](http://cjc.ict.ac.cn/online/onlinepaper/wc-202458160402.pdf)).

## First off, what is 'military-grade' encryption?
Honestly, I don't know. There is no such thing as 'military-grade' encryption. What most sensationalistic headlines are hinting at is the AES (Advanced Encryption Standard) block cipher, an industry standard and best practice among symmetric encryption algorithms [(ISO/IEC 18033-3)](https://www.iso.org/standard/54531.html). The 'military-grade' part is just a catchy and sexy addition that security consultants might use to extract more cash from their clients, I guess. Why? Well sure, the military uses it to protect the most jaw-dropping, sensitive, top-secret information out there. But with very high probability, your WiFi router is using it, too.

## Should you be worried?
The anwser is: "*probably not*".

First off, if **anyone on this planet** was able, as of today, of breaking AES, **for sure we wouldn't know of it**. Not now, not in 10 years, not in 20 years, maybe in 50 years. 
Think about it. The strategic advantage arising from the capability to break the algorithm that underpins any form of secure communication today (remember that AES makes HTTPS possible, which is what you use to connect to e-banking, your email, porn sites, and so on) would be tremendous. If a state actor (China, USA, Israel, you name it) had the technology to do so, it would undoubtedly go above and beyond to keep it secret for as long as possible.

Secondly, there is also a technical reason not to be so dramatic. The paper - quoting from [*The quantum insider*](https://thequantuminsider.com/2024/10/11/chinese-scientists-report-using-quantum-computer-to-hack-military-grade-encryption/) - "*targets the Present, Gift-64, and Rectangle algorithms, called key representatives of the Substitution-Permutation Network (SPN) structure*". What the paper is actually about is reframing cryptographic problems (in particular RSA and SPN block ciphers) as combinatorial optimization problems, and exploiting 'quantum annealing' (i.e., an optimization process to find the global minimum of an objective function) to solve them.
The claim on RSA is nothing to be worried about: they illustrate the factorization of a 22-bit number (`2269753` to be precise), which would take basically a blink of an eye on my Gen 10 ThinkPad. Let's pretend to take the claims on the block ciphers more seriously.

<div style="display: flex; justify-content: center;">
  <img src="/assets/images/spn.jpg" alt="alt text" title="Substitution-Permutation Network" style="max-width: 100%; height: auto;" />
</div>


## What is a Substitution-Permutation Network
A Substitution-Permutation Network (SPN) is a common technique for designing block ciphers. Essentially, it involves repeating two operations, 'Substitution boxes' (S-boxes) and 'Permutation boxes' (P-boxes), applied in rounds. The input of each round goes into the S-box, the output of the S-box feeds into the P-box, and the output of the P-box becomes the input for the next round's S-box.

As the name suggests, S-boxes are responsible for substituting the input bits in such a way that it becomes **very complex** to understand the mathematical relationship between the input and the output (this is referred to in cryptography as 'Confusion'). P-boxes, on the other hand, scramble the order of the output bits, ensuring that each input bit can affect any of the output bits ('Diffusion').

The key point is that there's a significant difference between what’s commonly referred to as 'military-grade' encryption, like AES, and the block ciphers discussed in the paper. While both AES and these algorithms (e.g., PRESENT) share a structural similarity (the SPN structure), that’s where the resemblance largely ends. For example, PRESENT is designed to be lightweight and is often used in environments where resources like processing power or memory are limited (IoT). It operates with fewer rounds and usually smaller key sizes compared to AES. But the heart of any block cipher lies in its S-boxes and P-boxes, and in this respect, the ciphers are fundamentally different. There’s no formal link between the security of these ciphers: breaking PRESENT does not imply any vulnerability in AES, and vice versa.

Finally, it is a well-known fact in cryptography that the threat posed to symmetric cryptography (and hash functions, n.d.r.) by quantum computers lies in the speedup achieved by Grover's algorithm, which essentially halves the bit-security of block ciphers. Hence, AES-256, the 'military-grade' encryption, will still offer around 128-bit security in the face of quantum adversaries, which is more than enough to sleep soundly at night.

## The key takeaway
The fact that AES wasn't considered in the paper says a lot: despite the buzz, our beloved 'military-grade' encryption today remains unaffected by these pretentious claims. In general, I would be extra skeptical of papers (let alone articles) making such bold statements, especially if they are not coming from [IACR](https://iacr.org/) (that's at least where the real cryptography happens nowadays).