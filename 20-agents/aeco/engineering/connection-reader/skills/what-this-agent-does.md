# What this agent does

`connection-reader` reads a **steel connection** out of an IFC file as geometry you can place — the
import half of "design a connection in one tool, apply it in another".

## The problem it solves

A connection (a base plate, a shear tab, a bolted moment end-plate) is usually designed and checked
in a specialist tool — IDEA StatiCa, or a detailer like Tekla — and exported as **IFC**. Bringing
that connection into another model normally means re-modelling it by hand. This agent instead reads
the connection's real geometry straight out of the IFC, so it can be dropped onto the matching joint.

## How it works

- **web-ifc does the parsing.** The bundled `web-ifc` WASM engine opens the IFC and tessellates the
  real geometry — extruded plates, faceted-brep welds, boolean-clip copes, mapped-item bolts. This
  agent does **not** re-implement an IFC geometry kernel; it only *groups* and *maps*.
- **Grouping into connections.** Elements are grouped per `IfcElementAssembly` that carries
  connection hardware (plates/bolts/welds). `list` returns the candidates (fast, no geometry);
  `extract` tessellates one.
- **AWARE's `mesh` primitive.** Each plate/bolt/weld comes back as a `kind:"mesh"` scene element —
  `positions` + `indices` in canonical **millimetres** — the same primitive `viewer-3d` renders and
  `ifc` writes. So an extracted connection flows straight through the rest of the substrate.

## What it is not

- Not a full IFC importer — it targets *connections* (the plate/bolt/weld hardware of an assembly),
  not whole buildings.
- Not a parametric recogniser — it returns geometry (`mesh`), not a recipe with editable parameters.
  Recognising "this is a base plate with these bolts" is a separate, later capability.
- Not a writer — pair it with the `ifc` agent to write a scene back out.
