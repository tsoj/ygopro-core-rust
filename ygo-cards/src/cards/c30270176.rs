//! Crimson Nova the Dark Cubic Lord
use super::prelude::*;

pub const ID: u32 = 30270176;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CUBIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be special Summon by its own method
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure (from hand)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
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
    // Unaffected by activated monster effects, whose original ATK is 3000 or less
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.immval)
    effect::set_value_fn(d, e3, lua_fn(immval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Make it be able to make a second attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Inflict 3000 damage to both players
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e5, CATEGORY_DAMAGE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.damcon)
    effect::set_condition(d, e5, Some(condition(damcon)));
    // e5:SetTarget(s.damtg)
    effect::set_target(d, e5, Some(cost_target(damtg)));
    // e5:SetOperation(s.damop)
    effect::set_operation(d, e5, Some(operation(damop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.spcfilter(c)
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CUBIC) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_CUBIC as u16]) && !card::is_public(d, c)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetClassCount(Card.GetCode)==#sg,sg:GetClassCount(Card.GetCode)~=#sg
    let (mmz, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    let n = group::get_class_count(d, sg, &|d, x| card::get_code(d, x)[0] as i64);
    (mmz && n == sg.len(), n != sg.len())
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let e = a.e(0);
    let c = effect::get_handler(d, e).nil();
    // if c==nil then return true end
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local hg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_HAND,0,c)
    let hg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_HAND, 0, c.into());
    // return aux.SelectUnselectGroup(hg,e,tp,3,3,s.rescon,0)
    matches!(
        aux::select_unselect_group(d, &hg, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local hg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_HAND,0,c)
    let hg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_HAND, 0, c.into());
    // local g=aux.SelectUnselectGroup(hg,e,tp,3,3,s.rescon,1,tp,HINTMSG_CONFIRM,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &hg,
        Some(e),
        tp,
        Some(3),
        Some(3),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // g:DeleteGroup()
}
// function s.immval(e,te)
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // return te:GetOwner()~=e:GetHandler() and te:IsMonsterEffect() and te:IsActivated()
    //   and te:GetOwner():GetBaseAttack()<=3000 and te:GetOwner():GetBaseAttack()>=0
    effect::get_owner(d, te) != effect::get_handler(d, e)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && {
            let owner = effect::get_owner(d, te).nil();
            card::get_base_attack(d, owner) <= 3000 && card::get_base_attack(d, owner) >= 0
        }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() and aux.bdcon(e,tp,eg,ep,ev,re,r,rp)
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h) && aux::bdcon().call(d, &[Value::Effect(a.e)]).truthy()
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToBattle() and not e:GetHandler():IsHasEffect(EFFECT_EXTRA_ATTACK) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_battle(d, h) && card::is_has_effect(d, h, EFFECT_EXTRA_ATTACK, None).is_empty();
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, c) {
        return;
    }
    // Can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3201)
    effect::set_description(d, e1, 3201);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,3000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 3000);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(tp,3000,REASON_EFFECT,true)
    duel::damage(d, a.tp, 3000, REASON_EFFECT, true, None);
    // Duel.Damage(1-tp,3000,REASON_EFFECT,true)
    duel::damage(d, 1 - a.tp, 3000, REASON_EFFECT, true, None);
    // Duel.RDComplete()
    duel::rd_complete(d);
}
