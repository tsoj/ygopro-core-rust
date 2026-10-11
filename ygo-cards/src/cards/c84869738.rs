//! Supreme Rage
use super::prelude::*;

pub const ID: u32 = 84869738;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ZARC],
    listed_series: &[SET_SUPREME_KING_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy as many monsters you control as possible, and if you do, Special Summon up to 4 "Supreme King Dragon" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 2 "Supreme King Dragon" monsters to 1 "Supreme King Dragon" Xyz Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.mattg)
    effect::set_target(d, e2, Some(cost_target(mattg)));
    // e2:SetOperation(s.matop)
    effect::set_operation(d, e2, Some(operation(matop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ZARC),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// aux.NOT(aux.FaceupFilter(Card.IsCode,CARD_ZARC))
fn not_zarc(d: &mut Duel, c: CardId) -> bool {
    !(card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC]))
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_SUPREME_KING_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.NOT(aux.FaceupFilter(Card.IsCode,CARD_ZARC)),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&not_zarc), tp, LOCATION_MZONE, 0, NO_EX);
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,g,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), (&g).into(), duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return Duel.GetMZoneCount(tp,g)>0
        duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
    }
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(aux.NOT(aux.FaceupFilter(Card.IsCode,CARD_ZARC)),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&not_zarc), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsCode,CARD_ZARC)),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&not_zarc), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
// s.exfilter1(c)
fn exfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFacedown() and c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ)
    card::is_location(d, c, LOCATION_EXTRA) && card::is_facedown(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)
}
// s.exfilter2(c)
fn exfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and (c:IsType(TYPE_LINK) or (c:IsFaceup() and c:IsType(TYPE_PENDULUM)))
    card::is_location(d, c, LOCATION_EXTRA) && (card::is_type(d, c, TYPE_LINK) || (card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)))
}
// s.rescon(ft1,ft2,ft3,ft4,ft): function(sg,e,tp,mg) ... return res, not res end
fn rescon_check(d: &mut Duel, sg: &CardSet, ft: [i32; 5]) -> (bool, bool) {
    let [ft1, _ft2, ft3, ft4, ft] = ft;
    // local exnpct=sg:FilterCount(s.exfilter1,nil,LOCATION_EXTRA)
    let exnpct = group::filter_count(d, sg, &exfilter1, NO_EX) as i32;
    // local expct=sg:FilterCount(s.exfilter2,nil,LOCATION_EXTRA)
    let expct = group::filter_count(d, sg, &exfilter2, NO_EX) as i32;
    // local mct=sg:FilterCount(aux.NOT(Card.IsLocation),nil,LOCATION_EXTRA)
    let mct = group::filter_count(d, sg, &|d, c| !card::is_location(d, c, LOCATION_EXTRA), NO_EX) as i32;
    // local exct=sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
    let _exct = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    // local groupcount=#sg
    let groupcount = sg.len() as i32;
    // local classcount=sg:GetClassCount(Card.GetCode)
    let classcount = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) as i32;
    // local res=ft3>=exnpct and ft4>=expct and ft1>=mct and ft>=groupcount and classcount==groupcount
    let res = ft3 >= exnpct && ft4 >= expct && ft1 >= mct && ft >= groupcount && classcount == groupcount;
    (res, !res)
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local dg=Duel.GetMatchingGroup(aux.NOT(aux.FaceupFilter(Card.IsCode,CARD_ZARC)),tp,LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, Some(&not_zarc), tp, LOCATION_MZONE, 0, NO_EX);
    // if Duel.Destroy(dg,REASON_EFFECT)==0 then return end
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCountFromEx(tp)
    let mut ft2 = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0;
    // local ft3=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    let mut ft3 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ), None).0;
    // local ft4=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_PENDULUM+TYPE_LINK)
    let mut ft4 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM + TYPE_LINK), None).0;
    // local ft=math.min(Duel.GetUsableMZoneCount(tp),4)
    let mut ft = duel::get_usable_m_zone_count(d, tp, None).min(4);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        // if ft1>0 then ft1=1 end
        if ft1 > 0 {
            ft1 = 1;
        }
        // if ft2>0 then ft2=1 end
        if ft2 > 0 {
            ft2 = 1;
        }
        // if ft3>0 then ft3=1 end
        if ft3 > 0 {
            ft3 = 1;
        }
        // if ft4>0 then ft4=1 end
        if ft4 > 0 {
            ft4 = 1;
        }
        // ft=1
        ft = 1;
    }
    // local ect=aux.CheckSummonGate(tp)
    // if ect then
    if let Some(ect) = aux::check_summon_gate_limit(d, tp) {
        // ft1 = math.min(ect, ft1)
        ft1 = std::cmp::min(ect as i32, ft1);
        // ft2 = math.min(ect, ft2)
        ft2 = std::cmp::min(ect as i32, ft2);
        // ft3 = math.min(ect, ft3)
        ft3 = std::cmp::min(ect as i32, ft3);
        // ft4 = math.min(ect, ft4)
        ft4 = std::cmp::min(ect as i32, ft4);
    }
    // local loc=0
    let mut loc = 0;
    // if ft1>0 then loc=loc|LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE end
    if ft1 > 0 {
        loc |= LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;
    }
    // if ft2>0 or ft3>0 or ft4>0 then loc=loc|LOCATION_EXTRA end
    if ft2 > 0 || ft3 > 0 || ft4 > 0 {
        loc |= LOCATION_EXTRA;
    }
    // if loc==0 then return end
    if loc == 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,loc,0,nil,e,tp)
    let necro = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp));
    let sg = duel::get_matching_group(d, Some(&necro), tp, loc as u32, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.rescon(ft1,ft2,ft3,ft4,ft),1,tp,HINTMSG_SPSUMMON)
    let limits = [ft1, ft2, ft3, ft4, ft];
    let rc = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        rescon_check(d, sg, limits)
    };
    let rg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(1), Some(ft as usize), Some(&rc), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SpecialSummon(rg,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon(d, rg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
}
// s.xyzfilter(c,tp)
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUPREME_KING_DRAGON) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_GRAVE|LOCATION_EXTRA,0,2,nil,tp,c)
    card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| matfilter(d, x, tp, c)), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 2, NO_EX)
}
// s.matfilter(c,tp,xyzc)
fn matfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsSetCard(SET_SUPREME_KING_DRAGON) and c:IsMonster()
    //     and c:IsFaceup() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])
        && card::is_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
// s.mattg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| xyzfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| xyzfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// s.matop(e,tp,eg,ep,ev,re,r,rp)
fn matop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.matfilter),tp,LOCATION_GRAVE|LOCATION_EXTRA,0,2,2,nil,tp,tc)
        let necro = aux::necro_valley_filter(move |d, c| matfilter(d, c, tp, tc));
        let g = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 2, 2, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Overlay(tc,g)
            duel::overlay(d, tc, g, false);
        }
    }
}
