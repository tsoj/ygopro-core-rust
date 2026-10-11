//! Exosisters Magnifica
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 59242457;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Rank 4 "Exorsister" Xyz Monsters
    // Xyz.AddProcedure(c,s.xyzfilter,nil,2,nil,nil,nil,nil,false)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| xyzfilter(d, c, sc, st, tp))), None, 2, None, None, None, None, false, None);
    // Must be Xyz Summoned using the correct materials
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 opponent card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return material to Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.tecon)
    effect::set_condition(d, e3, Some(condition(tecon)));
    // e3:SetTarget(s.tetg)
    effect::set_target(d, e3, Some(cost_target(tetg)));
    // e3:SetOperation(s.teop)
    effect::set_operation(d, e3, Some(operation(teop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and c:IsRank(4) and c:IsSetCard(SET_EXOSISTER,xyz,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, sc, st, Some(tp)) && card::is_rank(d, c, &[4]) && card::is_set_card_ctx(d, c, &[SET_EXOSISTER as u16], sc, st, Some(tp))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_XYZ)==SUMMON_TYPE_XYZ and not se
    let st = a.int(3);
    (st & SUMMON_TYPE_XYZ as i64) == SUMMON_TYPE_XYZ as i64 && !a.v(1).truthy()
}
/// `s.rmfilter(c)` (the Lua `Card.IsAbleToRemove` filter, with its defaults)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.tecon(e,tp,eg,ep,ev,re,r,rp)`
fn tecon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp
    a.rp == 1 - a.tp
}
/// `s.tefilter(c,tp)`
fn tefilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetOwner()==tp and c:IsType(TYPE_XYZ) and c:IsAbleToExtra()
    card::get_owner(d, c) == tp && card::is_type(d, c, TYPE_XYZ) && card::is_able_to_extra(d, c)
}
/// `s.tetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 and c:GetOverlayGroup():IsExists(s.tefilter,1,nil,tp) end
    if a.chk == Some(0) {
        let og = card::get_overlay_group(d, c);
        return card::get_flag_effect(d, c, ID) == 0 && group::is_exists(d, &og, &|d, c| tefilter(d, c, tp), 1, NO_EX);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_OVERLAY)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_OVERLAY as i32);
    true
}
/// `s.teop(e,tp,eg,ep,ev,re,r,rp)`
fn teop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local tc=c:GetOverlayGroup():FilterSelect(tp,s.tefilter,1,1,nil,tp):GetFirst()
    let og = card::get_overlay_group(d, c);
    let sel = group::filter_select(d, &og, tp, &|d, c| tefilter(d, c, tp), 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc or Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)<1 or not tc:IsLocation(LOCATION_EXTRA) then return end
    let Some(tc) = tc else {
        return;
    };
    if duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) < 1 || !card::is_location(d, tc, LOCATION_EXTRA) {
        return;
    }
    // if c:IsFacedown() or c:IsControler(1-tp) or c:IsImmuneToEffect(e) then return end
    if card::is_facedown(d, c) || card::is_controler(d, c, 1 - tp) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and not pg:IsContains(c)) then return end
    if group::get_count(&pg) > 1 || (group::get_count(&pg) == 1 && !group::is_contains(&pg, c)) {
        return;
    }
    // if tc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp)
    //     and c:IsCanBeXyzMaterial(tc,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,c,tc)>0
    //     and tc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_type_ctx(d, tc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_can_be_xyz_material(d, c, Some(tc), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(tc), None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local mg=Group.FromCards(c)
        let mg = CardSet::from_card(c);
        // tc:SetMaterial(mg)
        card::set_material(d, tc, mg.clone());
        // Duel.Overlay(tc,mg)
        duel::overlay(d, tc, mg, false);
        // if Duel.SpecialSummon(tc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, tc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
