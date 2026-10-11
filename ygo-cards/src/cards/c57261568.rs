//! Flower Cardian Peony with Butterfly
use super::prelude::*;

pub const ID: u32 = 57261568;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must first be Special Summoned (from your hand) by Tributing 1 "Flower Cardian" monster, except "Flower Cardian Peony with Butterfly"
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Draw 1 card, and if you do, show it, then, if it is a "Flower Cardian" monster, look at the top 3 cards of your opponent's Deck, then place them all on the top or bottom of the Deck in any order
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When this card is used as Synchro Material, you can treat it and all other Synchro Materials (that have a Level) as Level 2 monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e2, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e2:SetOperation(s.synop)
    effect::set_operation(d, e2, Some(lua_fn(synop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspfilter(c)`
fn selfspfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FLOWER_CARDIAN) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]) && !card::is_code(d, c, &[ID])
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.selfspfilter,1,false,1,true,c,tp,nil,false,nil)
    duel::check_release_group(
        d,
        tp,
        Some(&selfspfilter),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.SelectReleaseGroup(tp,s.selfspfilter,1,1,false,true,true,c,nil,nil,false,nil)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&selfspfilter),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: a.card(9), ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dn) = (info[0].int() as u8, info[1].int() as u32);
    // if Duel.Draw(p,d,REASON_EFFECT)>0 then
    if duel::draw(d, p, dn, REASON_EFFECT) > 0 {
        // local dc=Duel.GetOperatedGroup():GetFirst()
        let og = duel::get_operated_group(d);
        let dc = group::get_first(d, &og).nil();
        // Duel.ConfirmCards(1-tp,dc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(dc));
        // if dc:IsSetCard(SET_FLOWER_CARDIAN) and dc:IsMonster() then
        if card::is_set_card(d, dc, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, dc) {
            // local deck_ct=Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)
            let deck_ct = duel::get_field_group_count(d, 1 - tp, LOCATION_DECK, 0);
            // local ct=math.min(deck_ct,3)
            let ct = deck_ct.min(3);
            // if ct==0 then Duel.ShuffleHand(tp) return end
            if ct == 0 {
                duel::shuffle_hand(d, tp);
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local g=Duel.GetDecktopGroup(1-tp,ct)
            let g = duel::get_decktop_group(d, 1 - tp, ct as u32);
            // Duel.ConfirmCards(tp,g)
            duel::confirm_cards(d, tp, g);
            // local op=0
            let mut op = 0;
            // if deck_ct>3 then
            if deck_ct > 3 {
                // op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
                op = duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
            }
            // Duel.SortDecktop(tp,1-tp,ct)
            duel::sort_decktop(d, tp, 1 - tp, ct as u16);
            // if op==1 then Duel.MoveToDeckBottom(ct,1-tp) end
            if op == 1 {
                duel::move_decktop_to_bottom(d, ct as u32, 1 - tp);
            }
        } else {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(dc,REASON_EFFECT)
            duel::send_to_grave(d, CardSet::from_card(dc), REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    let sg = a.g(3);
    let lv = a.int(4) as u32;
    let sc = a.c(5);
    let n = sg.len() as i32;
    // local res=sg:CheckWithSumEqual(Card.GetSynchroLevel,lv,#sg,#sg,sc)
    //     or sg:CheckWithSumEqual(function() return 2 end,lv,#sg,#sg)
    let res = group::check_with_sum_equal(d, &sg, &|d, x| card::get_synchro_level(d, x, sc) as i64, lv, n, n).0
        || group::check_with_sum_equal(d, &sg, &|_d, _x| 2, lv, n, n).0;
    // return res,true
    Value::Multi(vec![Value::Bool(res), Value::Bool(true)])
}
