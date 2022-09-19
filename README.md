<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>


<!-- PROJECT SHIELDS -->

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/wynss/kesko">
    <img src="media/kesko-light.png" alt="Logo" width="200" height="200">
  </a>

<h3 align="center">Kesko</h3>

  <p align="center">
    Experimental robotics simulator written in Rust.
    <br />
    <br />
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/wynss/kesko/CI?style=flat-square">
    <img alt="GitHub" src="https://img.shields.io/github/license/wynss/kesko?style=flat-square">
    <br />
    <br />
    <a href="#about-the-project">About The Project</a> |
    <a href="#getting-started">Getting Started</a> |
    <a href="#demo">Demo</a> |
    <a href="#roadmap">Roadmap</a> |
    <a href="#acknowledgments">Acknowledgments</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About The Project
<img src="media/kesko-1.png" alt="drawing" style="width:500px;"/>
<br />

Kesko is pre-release and in its infancy, quite far from useful (but perhaps a bit fun to play around with). Currently it's more an exploration in Rust and what is currently possible within its open-source community. But, as time goes on the goal is to create a modern robotics simulator that fulfills

* Easiest simulator to get started with machine learning and robotics
* Best simulator for procedurally generate new environments for training
* Free and open-source
* Good looking!

Going forward an initial python API is priority so the first machine learning models can be trained.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started
### Prerequisites
 
 Make sure you have the latest stable version of [Rust](https://www.rust-lang.org/learn/get-started) installed.

### Installation

1. Clone the repo
   ```bash
   git clone https://github.com/wynss/kesko.git
   ```
2. Build and run
   ```bash
   cargo run --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

As stated Kesko is quite useless at the moment but you can play around with some of the built in models and experiment with the physics.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Demo

Thanks to WebAssembly support Kesko can run in the browser, try it at [Kesko Demo](https://toniaxelsson.com/projects/kesko)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

The roadmap is subject to change, but a very rough version is 

- [ ] Python API
    - [ ] GYM integration
    - [ ] Image data
- [ ] URDF Support
- [ ] Locally attached cameras

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Toni Axelsson - [@toniaxelsson](https://twitter.com/toniaxelsson)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Many thanks to the awesome people building
* [Bevy](https://bevyengine.org)
* [Rapier](https://rapier.rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[ci-shield]: https://img.shields.io/github/workflow/status/wynss/kesko/CI?style=flat-square
[product-screenshot]: media/kesko-1.png
