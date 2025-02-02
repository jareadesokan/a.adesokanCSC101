--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character(50),
    c_mobile text NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Mustafa Karim	35	m_karim@gmail.com                                 	8055089112	102	5
111	Lilian Jaiye	43	l_jaiye@gmail.com                                 	8055185431	102	3
112	Arthur Musa	40	a_musa@gmail.com                                  	7055282813	107	9
113	Philip Akonjo	41	p_akonjo@gmail.com                                	9052336772	102	2
114	Marylene Maps	39	m_maps@gmail.com                                  	8053335571	110	5
115	Oshenero Agor	32	o_agor@gmail.com                                  	7055667784	117	11
116	Adams Bree	30	a_bree@gmail.com                                  	8057684624	102	7
117	Okafor Mathias	38	o_mathias@gmail.com                               	8058736387	117	9
118	Samson Adeleke	65	s_adeleke@gmail.com                               	7056774423	110	11
119	Lawal Tamire	85	l_tamire@gmail.com                                	9052111101	107	5
120	James Job	44	j_job@gmail.com                                   	8056983919	110	7
121	Matthew Jakande	21	m_jakande@gmail.com                               	7051232144	102	9
122	Jimila Adegboye	20	j_adegboye@gmail.com                              	8054921923	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

