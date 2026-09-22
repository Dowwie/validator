<h1 align="center">Validator</h1>

<h3 align="center">Find the errors. Measure what changes.</h3>

<p align="center">
  A command-line tool that compares <strong>expected labels</strong> with
  <strong>generated outcomes</strong>.<br>
  Works with single-label and multi-label
  classifiers, including Jev workflows.
</p>

<p align="center">
  <strong>1. Bring your data.</strong> Pair reviewed expected labels with your workflow's saved predictions.<br>
  <strong>2. Find the mistakes.</strong> See missed labels, false alarms, and the cases behind them.<br>
  <strong>3. Check your changes.</strong> Compare runs to see what improved and what regressed.
</p>

<br><br>

<h1 align="center">Worked example<br> Evaluating a code-review workflow</h1>

<p align="center">
  <strong>87.1% micro-F1. Half the security findings missed.</strong><br>
  These 20 synthetic reviews show how Validator exposes a weakness behind the average.
</p>

<p align="center">
  <a href="assets/classification-matrix.html">
    <picture>
      <source media="(max-width: 600px)" srcset="assets/classification-matrix-mobile.png">
      <img src="assets/classification-matrix.png" width="100%" alt="Worked example: Validator evaluates 20 synthetic code reviews. Overall micro-F1 is 87.1%, but security recall is only 50%: three expected security findings were detected and three were missed.">
    </picture>
  </a>
</p>

<p align="center">
  <em>Synthetic example, not measured model performance. Figure based on Validator's JSON results.</em>
</p>

<br>

<p align="center">
  <a href="examples/readme-code-review/README.md"><strong>Try the worked example &rarr;</strong></a><br>
  Reproduce the results without model calls or provider credentials.<br>
  The guide explains the matrices, metrics, and commands.
</p>

<br>

<hr>

<p align="center">
  <a href="examples/jev-poc/PORTFOLIO.md">More use cases</a> &nbsp; &middot; &nbsp;
  <a href="schemas/">Input formats</a> &nbsp; &middot; &nbsp;
  <a href="architecture.md">Architecture</a> &nbsp; &middot; &nbsp;
  <a href="docs/specs/validator-v1.md">Specification</a>
</p>
