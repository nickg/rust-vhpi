module mixed_dut (
  input wire a,
  input wire b,
  input wire c,
  output wire y
);
  wire ab_xor;
  wire leaf_y;

  assign ab_xor = a ^ b;

  mixed_leaf u_leaf (
    .a(ab_xor),
    .b(c),
    .y(leaf_y)
  );

  assign y = leaf_y;
endmodule