//! Dark Creator
use super::prelude::*;

pub const ID: u32 = 52768390;
const TOKEN_DARK_CREATOR: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_DARK_CREATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by Tributing 2 Fiend and/or Fairy monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn: You can Special Summon as many "Dark Creator Tokens" (Fiend/DARK/Level 10/ATK 3000/DEF 1000) as possible, but they cannot attack directly or be destroyed by battle, destroy them when this card is destroyed, also you cannot Special Summon for the rest of this turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.tokensptg)
    effect::set_target(d, e3, Some(cost_target(tokensptg)));
    // e3:SetOperation(s.tokenspop)
    effect::set_operation(d, e3, Some(operation(tokenspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // If a monster(s) is destroyed by battle: Inflict 700 damage to your opponent
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// The release arguments of `Duel.CheckReleaseGroup(tp,Card.IsRace,2,false,2,true,c,tp,nil,false,nil,RACE_FIEND|RACE_FAIRY)`
fn tribute_args(c: Option<CardId>, tp: u8) -> duel::ReleaseArgs {
    duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(2),
        cancelable: false,
        check_field: true,
        to_check: c,
        toplayer: Some(tp),
        zone: None,
        use_oppo: false,
    }
}
/// `Card.IsRace` with the extra argument `RACE_FIEND|RACE_FAIRY`
fn fiend_or_fairy(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_FIEND | RACE_FAIRY)
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,Card.IsRace,2,false,2,true,c,tp,nil,false,nil,RACE_FIEND|RACE_FAIRY)
    duel::check_release_group(d, tp, Some(&fiend_or_fairy), 2, tribute_args(Some(c), tp), NO_EX, false)
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.card(9);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsRace,2,2,false,true,true,c,tp,nil,false,nil,RACE_FIEND|RACE_FAIRY)
    let mut ra = tribute_args(c, tp);
    ra.cancelable = true;
    ra.max = None;
    let g = duel::select_release_group(d, tp, Some(&fiend_or_fairy), 2, 2, ra, NO_EX, false);
    // if g then
    match g {
        Some(g) => {
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            // return true
            true
        }
        // return false
        None => false,
    }
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)
fn selfspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if g then
    if let Some(g) = g {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    }
}
// function s.tokensptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tokensptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DARK_CREATOR,0,TYPES_TOKEN,3000,1000,10,RACE_FIEND,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return ft > 0 && can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ft,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ft as u8, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ft,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ft as u8, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DARK_CREATOR,0,TYPES_TOKEN,3000,1000,10,RACE_FIEND,ATTRIBUTE_DARK)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_DARK_CREATOR,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(3000),
            defense: Some(1000),
            level: Some(10),
            race: Some(RACE_FIEND),
            attribute: Some(ATTRIBUTE_DARK),
        },
        None,
        None,
        0,
    )
}
// function s.tokenspop(e,tp,eg,ep,ev,re,r,rp)
fn tokenspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DARK_CREATOR,0,TYPES_TOKEN,3000,1000,10,RACE_FIEND,ATTRIBUTE_DARK) then
    if ft > 0 && can_summon_token(d, tp) {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local sg=Group.CreateGroup()
        let mut sg = CardSet::new();
        // for i=1,ft do
        for _ in 1..=ft {
            // local token=Duel.CreateToken(tp,TOKEN_DARK_CREATOR)
            let token = duel::create_token(d, tp, TOKEN_DARK_CREATOR).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // sg:AddCard(token)
            group::add_card(&mut sg, &CardSet::from_card(token));
            // --They cannot attack directly or be destroyed by battle
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3207)
            effect::set_description(d, e1, 3207);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetDescription(3000)
            effect::set_description(d, e2, 3000);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
        }
        // if c:IsRelateToEffect(e) and #sg>0 then
        if card::is_relate_to_effect(d, c, e) && !sg.is_empty() {
            // --Destroy them when this card is destroyed
            // local e3a=Effect.CreateEffect(c)
            let e3a = effect::create_effect(d, c);
            // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3a:SetCode(EVENT_LEAVE_FIELD_P)
            effect::set_code(d, e3a, EVENT_LEAVE_FIELD_P);
            // e3a:SetCondition(function() return c:IsReason(REASON_DESTROY) end)
            effect::set_condition(d, e3a, Some(condition(move |d, _a| card::is_reason(d, c, REASON_DESTROY))));
            // e3a:SetOperation(function()
            //     local e3b=Effect.CreateEffect(c) ... end)
            let sg_outer = sg.clone();
            effect::set_operation(d, e3a, Some(operation(move |d, _a| {
                // local e3b=Effect.CreateEffect(c)
                let e3b = effect::create_effect(d, c);
                // e3b:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e3b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
                // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e3b:SetCode(EVENT_DESTROYED)
                effect::set_code(d, e3b, EVENT_DESTROYED);
                let sg_inner = sg_outer.clone();
                // e3b:SetOperation(function(e) ... e:Reset() end)
                effect::set_operation(d, e3b, Some(operation(move |d, ea| {
                    // local dg=sg:Filter(Card.IsOnField,nil)
                    let dg = group::filter(d, &sg_inner, &|d, x| card::is_on_field(d, x), NO_EX);
                    // if #dg>0 then
                    if !dg.is_empty() {
                        // Duel.Hint(HINT_CARD,1-tp,id)
                        duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
                        // Duel.Destroy(dg,REASON_EFFECT)
                        duel::destroy(d, dg, REASON_EFFECT, None, None);
                    }
                    // e:Reset()
                    effect::reset(d, ea.e);
                })));
                // c:RegisterEffect(e3b)
                card::register_effect(d, c, e3b, false);
            })));
            // e3a:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3a, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e3a)
            card::register_effect(d, c, e3a, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // --You cannot Special Summon for the rest of this turn
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(700)
    duel::set_target_param(d, 700);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,700)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 700);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dam = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
