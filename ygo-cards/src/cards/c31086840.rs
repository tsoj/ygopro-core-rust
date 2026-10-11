//! Number 69: Heraldry Crest - Shatter Stream
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 31086840;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(69), listed_series: &[2407234, 77571454], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 4 Level 4 monsters OR 1 monster whose original name is "Number 69: Heraldry Crest"
    // Xyz.AddProcedure(c,nil,4,4,s.ovfilter,aux.Stringid(id,0),4)
    xyz::add_procedure(d, c, None, Some(4), 4, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), Some(xyz::MaxCt::N(4)), None, false, None);
    // Special Summon 1 "Number 69: Heraldry Crest - Dark Matter Demolition" from your Extra Deck and destroy an opponent's monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let attacker = duel::get_attacker(d).nil();
        card::is_controler(d, attacker, 1 - a.tp)
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, _tp: u8, _xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalCodeRule(2407234)
    card::is_faceup(d, c) && card::is_original_code_rule(d, c, &[2407234])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ttype,tloc,tplayer=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_TYPE,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CONTROLER)
    // return ttype&TYPE_MONSTER>0 and tloc&LOCATION_ONFIELD>0 and tplayer==1-tp
    let v = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    let ttype = v.first().map_or(0, |x| x.int());
    let tloc = v.get(1).map_or(0, |x| x.int());
    let tplayer = v.get(2).map_or(0, |x| x.int());
    ttype & TYPE_MONSTER as i64 > 0 && tloc & LOCATION_ONFIELD as i64 > 0 && tplayer == 1 - a.tp as i64
}
/// `s.xyzfilter(c,e,tp,mc)`
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsCode(77571454) and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_code(d, c, &[77571454])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c)))
        //     and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, a.e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e)) then return end
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, tp) && !card::is_immune_to_effect(d, c, a.e)) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and not pg:IsContains(c)) then return end
    if pg.len() > 1 || (pg.len() == 1 && !group::is_contains(&pg, c)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, a.e, tp, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(xyzc) = group::get_first(d, &sel) else { return };
    // xyzc:SetMaterial(c)
    card::set_material(d, xyzc, CardSet::from_card(c));
    // Duel.Overlay(xyzc,c)
    duel::overlay(d, xyzc, c, false);
    // if Duel.SpecialSummon(xyzc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, xyzc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // xyzc:CompleteProcedure()
        card::complete_procedure(d, xyzc);
        // local dg=Duel.GetFirstTarget()
        let dg = duel::first_target(d).nil();
        // if dg:IsControler(1-tp) and dg:IsRelateToEffect(e)
        //     and (Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) or dg:IsRelateToEffect(re)) then
        if card::is_controler(d, dg, 1 - tp)
            && card::is_relate_to_effect(d, dg, a.e)
            && (duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some() || a.re.map_or(false, |re| card::is_relate_to_effect(d, dg, re)))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
