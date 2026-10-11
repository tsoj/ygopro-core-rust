//! Flower Cardian Moonflowerviewing
use super::prelude::*;

pub const ID: u32 = 33541430;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),2,2)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 2, 2, None, None, None, None, None);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.drcost)
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // synchro level
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e3, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e3:SetOperation(s.synop)
    effect::set_operation(d, e3, Some(lua_fn(synop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, handler);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCode(EFFECT_SKIP_DP)
    effect::set_code(d, e1, EFFECT_SKIP_DP);
    // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dr = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)~=0 then
    if duel::draw(d, p, dr, REASON_EFFECT) != 0 {
        // local tc=Duel.GetOperatedGroup():GetFirst()
        let operated = duel::get_operated_group(d);
        let tc = group::get_first(d, &operated).nil();
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if tc:IsMonster() and tc:IsSetCard(SET_FLOWER_CARDIAN) then
        if card::is_monster(d, tc) && card::is_set_card(d, tc, &[SET_FLOWER_CARDIAN as u16]) {
            // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,1))
            //     and Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
            if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
                && duel::special_summon_step(d, tc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None)
            {
                // local e1=Effect.CreateEffect(e:GetHandler())
                let handler = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, handler);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DIRECT_ATTACK)
                effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e1,true)
                card::register_effect(d, tc, e1, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    }
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    // local res=sg:CheckWithSumEqual(Card.GetSynchroLevel,lv,#sg,#sg,sc)
    //     or sg:CheckWithSumEqual(function() return 2 end,lv,#sg,#sg)
    let sg = a.g(3);
    let lv = a.int(4) as u32;
    let sc = a.c(5);
    let n = sg.len() as i32;
    let res = group::check_with_sum_equal(d, &sg, &|d, x| card::get_synchro_level(d, x, sc) as i64, lv, n, n).0
        || group::check_with_sum_equal(d, &sg, &|_d, _x| 2, lv, n, n).0;
    // return res,true
    Value::Multi(vec![Value::Bool(res), Value::Bool(true)])
}
