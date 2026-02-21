library ieee;
use ieee.std_logic_1164.all;

entity tb_physical_time is
end entity;

architecture sim of tb_physical_time is
  type frequency_t is range 0 to 1000000000000000 units
    Hz;
    kHz = 1000 Hz;
    MHz = 1000 kHz;
    GHz = 1000 MHz;
    THz = 1000 GHz;
  end units;

  type voltage_t is range 0 to 5000 units
    mV;
    V = 1000 mV;
  end units;

  type frequency_vector_t is array (natural range <>) of frequency_t;
  type voltage_vector_t is array (natural range <>) of voltage_t;

  signal s_freq : frequency_t := 0 Hz;
  signal s_voltage : voltage_t := 0 mV;
  signal s_time : time := 0 ns;
  signal v_freq_arr : frequency_vector_t(0 to 2) := (0 Hz, 0 Hz, 0 Hz);
  signal v_voltage_arr : voltage_vector_t(0 to 2) := (0 mV, 0 mV, 0 mV);
  signal v_time_arr : time_vector(0 to 2) := (0 ns, 0 ns, 0 ns);
begin
  stim : process
  begin
    s_freq <= 100 MHz;
    s_voltage <= 3300 mV;
    s_time <= 100 ns;
    v_freq_arr <= (50 MHz, 100 MHz, 150 MHz);
    v_voltage_arr <= (1500 mV, 3300 mV, 5000 mV);
    v_time_arr <= (10 ns, 50 ns, 100 ns);
    wait for 10 ns;

    s_freq <= 500 MHz;
    s_voltage <= 2000 mV;
    s_time <= 250 ns;
    v_freq_arr <= (100 MHz, 250 MHz, 500 MHz);
    v_voltage_arr <= (2000 mV, 3300 mV, 3700 mV);
    v_time_arr <= (25 ns, 75 ns, 150 ns);
    wait for 10 ns;

    s_freq <= 100 THz;
    s_voltage <= 1800 mV;
    s_time <= 500 ns;
    v_freq_arr <= (250 MHz, 500 MHz, 1 GHz);
    v_voltage_arr <= (1200 mV, 2000 mV, 3300 mV);
    v_time_arr <= (50 ns, 100 ns, 200 ns);
    wait for 10 ns;

    wait;
  end process;
end architecture;
