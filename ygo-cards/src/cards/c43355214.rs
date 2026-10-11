//! The Undying Legion
use super::prelude::*;

pub const ID: u32 = 43355214;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure: 2 Level 7 monsters, OR 1 Rank 6 Zombie Xyz Monster you control
    // Xyz.AddProcedure(c,nil,7,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // --During your opponent's Main Phase (Quick Effect): You can detach 2 materials from this card (or just 1 material if all this card's materials are Xyz Monsters), then target 1 Attack Position monster your opponent controls or 1 monster in their GY; attach it to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsMainPhase(1-tp) end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetCost(Cost.DetachFromSelf(s.attachcostmin,2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::F(std::sync::Arc::new(attachcostmin)), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsRank(6) and c:IsRace(RACE_ZOMBIE,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsXyzMonster() and c:IsFaceup()
    card::is_rank(d, c, &[6])
        && card::is_race_ctx(d, c, RACE_ZOMBIE, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_xyz_monster(d, c)
        && card::is_faceup(d, c)
}
// function s.xyzop(e,tp,chk)
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    true
}
// function(e,tp) return Duel.IsMainPhase(1-tp) end
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_main_phase(d, Some(1 - tp))
}
// function s.attachcostmin(e,tp,eg,ep,ev,re,r,rp,chk)
fn attachcostmin(d: &mut Duel, e: EffectId, _tp: u8) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup()
    let handler = effect::get_handler(d, e).nil();
    let g = card::get_overlay_group(d, handler);
    // return #g==g:FilterCount(Card.IsXyzMonster,nil) and 1 or 2
    if g.len() == group::filter_count(d, &g, &|d, c| card::is_xyz_monster(d, c), NO_EX) {
        1
    } else {
        2
    }
}
// function s.attachfilter(c,xyzc,tp)
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsAttackPos() and c:IsMonster() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_attack_pos(d, c) && card::is_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
// function s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.attachfilter(chkc,c,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE)
            && attachfilter(d, chkc, c, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.attachfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.attachfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil,c,tp)
    duel::select_target(d, a.tp, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
// function s.attachop(e,tp,eg,ep,ev,re,r,rp)
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
            // Duel.Overlay(c,tc,true)
            duel::overlay(d, c, tc, true);
        }
    }
}
