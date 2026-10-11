//! Ghostrick Renovation
use super::prelude::*;

pub const ID: u32 = 61818176;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GHOSTRICK],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 "Ghostrick" Field Spell to the hand and activate 1 Field Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon 1 "ghostrick" Xyz monster from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return tc and tc:IsFaceup() and tc:IsSetCard(SET_GHOSTRICK) and tc:IsAbleToHand() and tc:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => {
                card::is_faceup(d, tc)
                    && card::is_set_card(d, tc, &[SET_GHOSTRICK as u16])
                    && card::is_able_to_hand(d, tc, None)
                    && card::is_can_be_effect_target(d, tc, Some(a.e))
            }
            None => false,
        };
    }
    // if not Duel.CheckPhaseActivity() then Duel.RegisterFlagEffect(tp,CARD_MAGICAL_MIDBREAKER,RESET_CHAIN,0,1) end
    if !duel::check_phase_activity(d) {
        duel::register_flag_effect(d, tp, CARD_MAGICAL_MIDBREAKER, RESET_CHAIN, 0, 1, 0);
    }
    // Duel.SetTargetCard(tc)
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0)
    match tc {
        Some(tc) => {
            duel::set_target_card(d, tc);
            duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, 0, 0);
        }
        None => {
            duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, 0);
        }
    }
    true
}
// function s.actfilter(c,tp)
fn actfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFieldSpell() and c:GetActivateEffect():IsActivatable(tp,true,true)
    card::is_field_spell(d, c) && {
        let ae = card::get_activate_effect(d, c).first().copied().nil();
        effect::is_activatable(d, ae, tp, true, true)
    }
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_HAND) then
    let Some(tc) = tc else {
        return;
    };
    if !(card::is_relate_to_effect(d, tc, e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_HAND))
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.actfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| actfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, sc, a, None);
    }
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let eg = CardSet::from_card(c);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&eg), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and c:IsSetCard(SET_GHOSTRICK)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetCode(),pg)
    let ok = (pg.len() == 0 || (pg.len() == 1 && group::is_contains(&pg, c)))
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_GHOSTRICK as u16]);
    if !ok {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, c, code)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.filter2(c,e,tp,mc,code,pg)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, code: u32) -> bool {
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_GHOSTRICK) and not c:IsCode(code)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and mc:IsCanBeXyzMaterial(c,tp)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
        && !card::is_code(d, c, &[code])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCountFromEx(tp,tp,tc,TYPE_XYZ)<=0 then return end
    let ex = match tc {
        Some(t) => t.into(),
        None => NO_EX,
    };
    if duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Type(TYPE_XYZ), None).0 <= 0 {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let tg = tc.map(CardSet::from_card).unwrap_or_default();
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&tg), tp, None, None, REASON_XYZ);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e)
    //     or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let Some(tc) = tc else {
        return;
    };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetCode(),pg)
    let code = card::get_code(d, tc)[0];
    let g = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, e, tp, tc, code)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
