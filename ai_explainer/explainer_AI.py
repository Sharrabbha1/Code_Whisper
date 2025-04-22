import json

# ✍️ Hardcoded explanations for common warning messages
EXPLANATIONS = {
    "Use of eval/exec is dangerous":
        "The eval() and exec() functions can run arbitrary code. If attackers control the input, they could run harmful commands.",
    "Possible hardcoded password":
        "Hardcoding passwords in code is risky. It can lead to credential leaks if the code is shared or deployed."
}

def explain_warning(warning):
    msg = warning["message"]
    explanation = EXPLANATIONS.get(msg, "No explanation available.")
    warning["explanation"] = explanation
    return warning

def main():
    input_file = "../output/result.json"
    output_file = "../output/result_with_explanations.json"

    with open(input_file, "r") as f:
        warnings = json.load(f)

    explained = [explain_warning(w) for w in warnings]

    with open(output_file, "w") as f:
        json.dump(explained, f, indent=2)

    print("✅ AI explanations added. Check output/result_with_explanations.json")

if __name__ == "__main__":
    main()
