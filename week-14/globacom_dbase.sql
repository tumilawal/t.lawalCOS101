--
-- PostgreSQL database dump
--

\restrict Y7vpC4MYw9BB1WqtHtaqE4bKSXZa8lQkxdvuCT1otYYwclbVvO7vrxKWiM6afmn

-- Dumped from database version 18.7
-- Dumped by pg_dump version 18.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer,
    dname text,
    dlocation text,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: department_dept_managerid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.department_dept_managerid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.department_dept_managerid_seq OWNER TO postgres;

--
-- Name: department_dept_managerid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.department_dept_managerid_seq OWNED BY public.department.dept_managerid;


--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name text,
    dno integer,
    staff_sal integer,
    age integer,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Name: employees_eid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.employees_eid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.employees_eid_seq OWNER TO postgres;

--
-- Name: employees_eid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.employees_eid_seq OWNED BY public.staff.staff_id;


--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer,
    pname text,
    pduration text,
    project_managerid integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: department dept_managerid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department ALTER COLUMN dept_managerid SET DEFAULT nextval('public.department_dept_managerid_seq'::regclass);


--
-- Name: staff staff_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff ALTER COLUMN staff_id SET DEFAULT nextval('public.employees_eid_seq'::regclass);


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	\N
101	2	Account	Egbeda	\N
100	3	Packaging	Ajah	\N
120	4	Research	V.I	\N
97	5	Account	Magodo	\N
122	6	Operations	Mile 2	\N
107	7	Packaging	Ketu	\N
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000	32	8063285831
107	Alokwe Martin	7	380000	48	7090082812
97	Dankade Aminat	5	550000	40	9023688832
108	Josiah Joshua	1	120000	30	8053189131
102	Mankinde Mary	2	450000	55	9023487830
120	Adeleke Jane	4	200000	38	7061045862
122	Osahon Mark	6	320000	44	8022289842
117	Suleman Ajayi	3	800000	50	7030089811
104	Kuti Lawal	1	750000	35	9145689842
\.


--
-- Name: department_dept_managerid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.department_dept_managerid_seq', 1, false);


--
-- Name: employees_eid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.employees_eid_seq', 1, false);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict Y7vpC4MYw9BB1WqtHtaqE4bKSXZa8lQkxdvuCT1otYYwclbVvO7vrxKWiM6afmn

