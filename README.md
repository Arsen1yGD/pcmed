# pcmed

Attempt to make .wav -> Floatbeat converter

# How to use

Run:
`$ ./pcmed <path_to_wav_file>`

This will generate an UTF-16 encoded .js file near .wav file with the same name. You need to put .js file contents in some bytebeat composer (like https://dollchan.net/bytebeat)
(pray to god to open .js file with UTF-16, copy it's contents and paste them to bytebeat composer without crashing)

# Build

<p>Windows: IDK, I don't use windows.<p/>

Linux:<br/>
`$ git clone https://github.com/Arsen1yGD/pcmed`<br/>
`$ cd pcmed`<br/>
`$ cargo build --release`<br/>

Chained into 1 command:

`$ git clone https://github.com/Arsen1yGD/pcmed && cd pcmed && cargo build --release`

# How it works

It opens a .wav file, reads it's samples, and writing them in a template:<br/>
<code>// Set sample rate to {sample rate}&#10;t||(c=\`{samples converted to utf-16 string}\`),c.charCodeAt(t%c.length)/32768-1</code>.

