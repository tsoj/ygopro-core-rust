//! Dark Ceruvim Irisfil
use super::prelude::*;

pub const ID: u32 = 64626565;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 8 monsters, OR 1 Xyz Monster you control that has not activated its effect in the Monster Zone this turn
    // Xyz.AddProcedure(c,nil,8,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(8),
        2,
        Some(xyz::alter_filter(|d, x, _tp, _xyzc| ovfilter(d, x))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Keep track of Xyz Monsters that have activated their effect in the Monster Zone each turn
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_CHAIN_SOLVED)
    //     ge1:SetOperation(s.checkop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_CHAIN_SOLVED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
    // If this card is Xyz Summoned: You can activate this effect; this turn, Xyz Monsters you control gain ATK equal to their own Rank x 100 during the Battle Phase only
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(xyz_summoned_cond)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card would be destroyed by battle or card effect, you can detach 1 material from a monster you control instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsXyzMonster() and not c:HasFlagEffect(id) and c:IsFaceup()
    card::is_xyz_monster(d, c) && !card::has_flag_effect(d, c, ID, None) && card::is_faceup(d, c)
}
// function s.xyzop(e,tp,chk)
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0).is_some()
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if re:IsMonsterEffect() and rc:IsXyzMonster() and rc:IsRelateToEffect(re)
    //     and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE then
    if effect::is_monster_effect(d, re)
        && card::is_xyz_monster(d, rc)
        && card::is_relate_to_effect(d, rc, re)
        && duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_MZONE as i64
    {
        // rc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, rc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
// function(e) return e:GetHandler():IsXyzSummoned() end
fn xyz_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // This turn, Xyz Monsters you control gain ATK equal to their own Rank x 100 during the Battle Phase only
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTarget(function(e,c) return c:IsXyzMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_xyz_monster(d, c))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(battle_phase_cond)));
    // e1:SetValue(function(e,c) return c:GetRank()*100 end)
    effect::set_value_fn(d, e1, effect_card_value(rank_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function() return Duel.IsBattlePhase() end
fn battle_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
// function(e,c) return c:GetRank()*100 end
fn rank_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_rank(d, c) as i64 * 100
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    //     and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
            && !card::is_reason(d, c, REASON_REPLACE)
            && duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, None);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, tp, c, Some(96)) && duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None) > 0
}
