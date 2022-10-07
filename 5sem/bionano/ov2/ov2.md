# Oving 2

## Oppg 1

Den en-dimensjonelle diffusjonsligningen:

$$
\frac{\partial c(x, t)}{\partial t} = D \frac{\partial^2 c(x, t)}{\partial x^2}
$$

### 1a)

Fourier-transformer begge sider:

$$
\int_{-\infty}^\infty \frac{\partial c(x, t)}{\partial t} \cdot e^{-i2\pi k x} dx 

= D \int_{-\infty}^\infty \frac{\partial^2 c(x, t)}{\partial x^2} \cdot e^{-i2\pi k x} dx 
$$

Venstre side: 

$$
\frac{1}{\sqrt{2\pi}} \int_{-\infty}^\infty \frac{\partial c(x, t)}{\partial t} \cdot e^{-i k x} dx 
$$

$$
= \frac{\partial}{\partial t} \frac{1}{\sqrt{2\pi}} \int_{-\infty}^\infty  c(x, t) \cdot e^{-i k x} dx 
$$

$$
= \frac{\partial}{\partial t} \tilde{c}(k, t)
$$

der $\tilde{c}(k, t)$ er Den fouriertransformerte $c(x, t)$

Høyre side: 

$$
D \frac{1}{\sqrt{2\pi}} \int_{-\infty}^\infty \frac{\partial^2 c(x, t)}{\partial x^2} \cdot e^{-i k x} dx 
$$

Delvis integrasjon:

$$
\int F \frac{\partial G}{\partial x} dx = (FG)\bigg|_{-\infty}^{+\infty} - \int \frac{\partial F}{\partial x}G dx
$$

Anta at $c(\plusmn \infty, t) = 0$ som gir mening når vi har en punktkilde. Dvs at $(FG)$-leddet alltid blir $0$. Deriveringen av $e^{-ikx}$ henter bare ut en konstant $(-ik)$


$$
= D \frac{1}{\sqrt{2\pi}} \left(0 - \int_{-\infty}^\infty \frac{\partial c(x, t)}{\partial x} \cdot e^{-i k x} \cdot (-ik) dx \right)
$$

$$
= D \frac{1}{\sqrt{2\pi}} (ik) \int_{-\infty}^\infty \frac{\partial c(x, t)}{\partial x} \cdot e^{-i k x} dx
$$

etter samme logikk:

$$
= D  (ik)^2 \frac{1}{\sqrt{2\pi}} \int_{-\infty}^\infty c(x, t) \cdot e^{-i k x} dx
$$

$$
= -D k^2 \tilde{c}(k, t) 
$$

Det finnes visst en regneregel for fouriertransformasjoner som viser det jeg viste over  
VS = HS

$$
\frac{\partial}{\partial t} \tilde{c}(k, t) = -D k^2 \tilde{c}(k, t) 
$$

Og vi har diff-ligningen vi var ute etter.

### 1b)

$$
\frac{\partial}{\partial t} \tilde{c}(k, t) = -D k^2 \tilde{c}(k, t) 
$$

matte-lifehack

$$
\frac{1}{\tilde{c}(k, t)} \partial\tilde{c}(k, t) = -D k^2  \partial t
$$

$$
\int \frac{1}{\tilde{c}(k, t)} \partial\tilde{c}(k, t) = \int -D k^2  \partial t
$$

$$
\ln (\tilde{c}(k, t)) =  -D k^2 \int \partial t = -D k^2 t + C_1
$$

$$
\tilde{c}(k, t) = e^{-D k^2 t} e^{C_1} = C_2e^{-D k^2 t}
$$

Grensebetingelse for å finne $C_2$:  
Vi vet at $c(x, 0) = \delta(0 \cdot c_0)$. Merk at i oppgaveteksten kaller de $c_0$ for $c$.

$$
\tilde{c}(k, 0) = \frac{1}{\sqrt{2\pi}} \int_{-\infty}^{+\infty} e^{ikx}\cdot c(x, 0) dx
$$

$$
 = \frac{1}{\sqrt{2\pi}} \left( e^{ikx} \cdot c_0 \right) \bigg|^{x=0} = \frac{c_0}{\sqrt{2\pi}}
$$

Sett inn i funksjonen fra over:

$$
\tilde{c}(k, 0) = C_2 \cdot 1 = \frac{c_0}{\sqrt{2\pi}}
$$

$$
\implies C_2 = \frac{c_0}{\sqrt{2\pi}}
$$

$$
\implies \tilde{c}(k, t) = \frac{c_0}{\sqrt{2\pi}} e^{-D k^2 t}
$$

Fouriertransformere tilbake: 

$$
c(x, t) = \frac{1}{\sqrt{2\pi}} \int_{-\infty}^{+\infty} \frac{c_0}{\sqrt{2\pi}} e^{-D k^2 t} \cdot e^{ikx} dk
$$

$$
= \frac{c_0}{\sqrt{4\pi^2}} \int_{-\infty}^{+\infty} e^{-D k^2 t + ikx} dk
$$

Regel fra rottmann s 155

$$
 \int_{-\infty}^{+\infty} e^{-ax^2 - bx - c} dx = \sqrt{\frac{\pi}{a}} \exp\left(\frac{b^2-ac}{a}\right)
$$

I vårt tilfelle er $a = Dt$, $b = \frac{-ix}{2}$, $c = 0$

$$
c(x, t) = \frac{c_0}{\sqrt{4\pi^2}} \sqrt{\frac{\pi}{Dt}}\exp\left(\frac{-x^2}{4Dt}\right) = \frac{c_0}{\sqrt{4\pi Dt}}e^{-x^2/Dt}
$$

### 1c)

Her er det ikke så godt å vite hva man skal frem til dessverre

Vi begynner med at 

$$
c(x, t) = \mathcal{F}^{-1}[\tilde{c}(k, t)] = \mathcal{F}^{-1}[e^{-Dk^2t} \cdot \tilde{c}(k, 0)]
$$

konvolusjonsteoremet sier at 

$$
\mathcal{F}^{-1}[G \cdot H] = \mathcal{F}^{-1}[G] * \mathcal{F}^{-1}[H]
$$

Dermed har vi 

$$
c(x, t) = \mathcal{F}^{-1}[e^{-Dk^2t}] * \mathcal{F}^{-1}[\tilde{c}(k, 0)]
$$

Første ledd i konvolusjonen

$$
\mathcal{F}^{-1}[e^{-Dk^2t}] = \frac{1}{\sqrt{2\pi}} \int_{-\infty}^{+\infty} e^{-Dk^2t}e^{ikt} dk
$$

som etter samme rottmann-formel som i oppgave 1b blir

$$
\mathcal{F}^{-1}[e^{-Dk^2t}] = \frac{1}{\sqrt{2\pi}} \sqrt{\frac{\pi}{Dt}}\exp\left(\frac{-x^2}{4Dt}\right) = \frac{1}{\sqrt{2Dt}}e^{-x^2/4Dt}
$$

For en eller annen grunn mangler angivelig jeg et ledd $1/\sqrt{2\pi}$ TODO   
Andre ledd i konvolusjonen:  

$$
\mathcal{F}^{-1}[\tilde{c}(k, 0)] = c(x, 0)
$$

per definisjon. Dermed får vi at (herfra og ned er korrigert for en feil over) TODO

$$
c(x, t) = \mathcal{F}^{-1}[e^{-Dk^2t}] * \mathcal{F}^{-1}[\tilde{c}(k, 0)] = c(x, 0) * \frac{1}{\sqrt{4\pi Dt}}e^{-x^2/4Dt}
$$

$$
= \int_{-\infty}^{+\infty} c(z - x, 0) \cdot \frac{1}{\sqrt{4\pi Dt}}e^{-z^2/4Dt} dz
$$ 

### 1d)

Jeg nekter å kalle denne oppgaven 1B)

$$
c(x, 0) = 
\begin{cases}
c_0 & \text{når $x < 0$}\\
0 & \text{ellers}
\end{cases}
$$

$$
c(x -z, 0) = 
\begin{cases}
c_0 & \text{når $x -z < 0$}\\
0 & \text{ellers}
\end{cases}
$$

dermed:

$$
c(x,t) = \int_{-\infty}^{+\infty} c(z - x, 0) \cdot \frac{1}{\sqrt{4\pi Dt}}e^{-z^2/4Dt} dz
$$

$$
= \int_{x}^{+\infty} c(z - x, 0) \cdot \frac{1}{\sqrt{4\pi Dt}}e^{-z^2/4Dt} dz
$$

Her kunne vi trengt mellomregning TODO

$$
=\left[
\frac{c_0\sqrt{pi}}{\sqrt{4\pi Dt}} \frac{\sqrt{4Dt}}{2}
erf\left(
\frac{1}{\sqrt{4Dt}}z
\right)
\right]_x^\infty
$$

$$
=\left[
\frac{c_0}{2}
erf\left(
\frac{1}{\sqrt{4Dt}}z
\right)
\right]_x^\infty
$$

$$
\frac{c_0}{2} - \frac{c_0}{2} erf\left(
\frac{1}{\sqrt{4Dt}}x
\right)
$$

