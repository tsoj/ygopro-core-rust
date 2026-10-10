//! Raviel, Lord of Phantasms
use super::prelude::*;

pub const ID: u32 = 69890967;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[69890968],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon Tokens
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.tkcon)
    effect::set_condition(d, e3, Some(condition(tkcon)));
    // e3:SetTarget(s.tktg)
    effect::set_target(d, e3, Some(cost_target(tktg)));
    // e3:SetOperation(s.tkop)
    effect::set_operation(d, e3, Some(operation(tkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Increase ATK
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.atcost)
    effect::set_cost(d, e4, Some(cost_target(atcost)));
    // e4:SetOperation(s.atop)
    effect::set_operation(d, e4, Some(operation(atop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.rfilter(c,tp)`
fn rfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND) and (c:IsControler(tp) or c:IsFaceup())
    card::is_race(d, c, RACE_FIEND) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp):Filter(s.rfilter,nil,tp)
    let rg0 = duel::get_release_group(d, tp, false, false, None);
    let rg = group::filter(d, &rg0, &|d, x| rfilter(d, x, tp), NO_EX);
    // return aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),0)
    let mmz = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&mmz), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetReleaseGroup(tp):Filter(s.rfilter,nil,tp)
    let rg0 = duel::get_release_group(d, tp, false, false, None);
    let rg = group::filter(d, &rg0, &|d, x| rfilter(d, x, tp), NO_EX);
    // local mg=aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let mmz = aux::chkf_mmz(1);
    let mg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&mmz), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #mg==3 then
    if mg.len() == 3 {
        // mg:KeepAlive()
        // e:SetLabelObject(mg)
        effect::set_label_object(d, e, Value::Group(mg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,1000,1000,1,RACE_FIEND,ATTRIBUTE_DARK) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            a.tp,
            ID + 1,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(1000),
                defense: Some(1000),
                level: Some(1),
                race: Some(RACE_FIEND),
                attribute: Some(ATTRIBUTE_DARK),
            },
            None,
            None,
            0,
        )
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetTextAttack()>0
    card::get_text_attack(d, c) > 0
}
/// `s.atcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.atkfilter,1,false,nil,c) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&atkfilter), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.atkfilter,1,1,false,nil,c)
    let g = aux::select_release_group_cost(d, a.tp, Some(&atkfilter), 1, 1, false, None, c.into());
    // local atk=g:GetFirst():GetTextAttack()
    let first = group::get_first(d, &g).nil();
    let mut atk = card::get_text_attack(d, first) as i64;
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // e:SetLabel(atk)
    effect::set_label(d, a.e, &[atk]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
