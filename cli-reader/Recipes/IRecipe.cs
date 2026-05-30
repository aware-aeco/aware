namespace AwareReader.Recipes;

/// <summary>
/// A recipe recognises an ecosystem idiom in the reflected IR and rewrites the default
/// per-method agent into idiomatic commands (#180). It is purely builder-side and runs
/// identically for compiled (sidecar) and source (roslyn) inputs, since both produce the
/// same <see cref="ReflectedSet"/>.
/// </summary>
public interface IRecipe
{
    /// <summary>Stable recipe name (for diagnostics).</summary>
    string Name { get; }

    /// <summary>True if this recipe recognises the reflected set.</summary>
    bool Matches(ReflectedSet set);

    /// <summary>Rewrite the baseline agent for the recognised idiom.</summary>
    GeneratedAgent Apply(ReflectedSet set, GeneratedAgent baseline);
}

/// <summary>
/// The recipe registry. <see cref="Detect"/> returns the first recipe that matches the set,
/// or null for the default (per-method) synthesis. Order matters only if recipes overlap;
/// today there is a single recipe (Tekla model plug-ins).
/// </summary>
public static class RecipeRegistry
{
    private static readonly IRecipe[] All = [new TeklaPluginRecipe()];

    public static IRecipe? Detect(ReflectedSet set) => Array.Find(All, r => r.Matches(set));
}
